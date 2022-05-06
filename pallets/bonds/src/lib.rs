//! Pallet to manage the Bonds (Debit Market) on BitGreen Blockchain
#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use core::str;
use core::str::FromStr;
use frame_support::{codec::Decode, dispatch, ensure, traits::Currency};
use frame_system::RawOrigin;
use frame_system::{ensure_root, ensure_signed};
use sp_runtime::traits::StaticLookup;
use sp_std::borrow::ToOwned;
use sp_std::prelude::*;
pub type Balance = u128;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_assets::Config<AssetId = u32, Balance = u128>
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId, Balance = Balance>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    // Settings configuration, we store json structure with different keys (see the function for details)
    #[pallet::storage]
    #[pallet::getter(fn get_settings)]
    pub type Settings<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>>;

    // Know Your client Data
    #[pallet::storage]
    #[pallet::getter(fn get_kyc)]
    pub type Kyc<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>>;

    #[pallet::storage]
    #[pallet::getter(fn get_kycsignatures)]
    pub type KycSignatures<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, u32>;

    #[pallet::storage]
    #[pallet::getter(fn get_kycapproved)]
    pub type KycApproved<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

    #[pallet::storage]
    #[pallet::getter(fn get_bond)]
    pub type Bonds<T: Config> = StorageMap<_, Blake2_128Concat, u32, Vec<u8>>;

    #[pallet::storage]
    #[pallet::getter(fn get_bondssignatures)]
    pub type BondsSignatures<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, T::AccountId, u32>;

    #[pallet::storage]
    #[pallet::getter(fn get_bondapproved)]
    pub type BondsApproved<T: Config> = StorageMap<_, Blake2_128Concat, u32, u32>;

    // Bond Shares (subscription )
    #[pallet::storage]
    #[pallet::getter(fn get_bondsshares)]
    pub type BondsShares<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, T::AccountId, u32>;

    // Bond Total Subscribed
    #[pallet::storage]
    #[pallet::getter(fn get_bondstotalshares)]
    pub type BondsTotalShares<T: Config> = StorageMap<_, Blake2_128Concat, u32, u32>;

    // Credit Rating Agencies
    #[pallet::storage]
    #[pallet::getter(fn get_creditrating_agency)]
    pub type CreditRatingAgencies<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>>;

    #[pallet::storage]
    #[pallet::getter(fn get_creditrating)]
    pub type CreditRatings<T: Config> = StorageMap<_, Blake2_128Concat, u32, Vec<u8>>;

    // Collaterals
    #[pallet::storage]
    #[pallet::getter(fn get_collateral)]
    pub type Collaterals<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, u32, Vec<u8>>;

    // Collaterals
    #[pallet::storage]
    #[pallet::getter(fn get_collateral_approval)]
    pub type CollateralsApproval<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, u32, Vec<u8>>;

    #[pallet::storage]
    #[pallet::getter(fn get_fund)]
    pub type Funds<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>>;

    #[pallet::storage]
    #[pallet::getter(fn get_fund_signatures)]
    pub type FundsSignatures<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, u32>;

    #[pallet::storage]
    #[pallet::getter(fn get_fund_approved)]
    pub type FundsApproved<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

    // Standard Iso country code and official name
    #[pallet::storage]
    #[pallet::getter(fn get_iso_country)]
    pub type IsoCountries<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>>;

    // Currencies data
    #[pallet::storage]
    #[pallet::getter(fn get_currency)]
    pub type Currencies<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>>;

    // Underwriters data
    #[pallet::storage]
    #[pallet::getter(fn get_underwriter)]
    pub type Underwriters<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>>;

    // Insurers data
    #[pallet::storage]
    #[pallet::getter(fn get_insurer)]
    pub type Insurers<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>>;

    // Insurances data
    #[pallet::storage]
    #[pallet::getter(fn get_insurance)]
    pub type Insurances<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, Vec<u8>>;

    //Frozen funds in an Pool Account according to the percentage of mandatory reserves
    #[pallet::storage]
    #[pallet::getter(fn get_insurer_reserves)]
    pub type InsurerReserves<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Balance, ValueQuery>;

    // Insurances Signed from Payer
    #[pallet::storage]
    #[pallet::getter(fn get_insurance_signature)]
    pub type InsurancesSigned<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, T::AccountId>;

    // Lawyers data
    #[pallet::storage]
    #[pallet::getter(fn get_lawyer)]
    pub type Lawyers<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<u8>>;

    // InterbankRate data
    #[pallet::storage]
    #[pallet::getter(fn get_interbank_rate)]
    pub type InterbankRates<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, Vec<u8>, Blake2_128Concat, Vec<u8>, u32>;

    // InterbankRate data
    #[pallet::storage]
    #[pallet::getter(fn get_inflation_rate)]
    pub type InflationRates<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, Vec<u8>, Blake2_128Concat, Vec<u8>, u32>;

    #[pallet::storage]
    #[pallet::getter(fn get_insurer_risks_covered)]
    pub type InsurerRisksCovered<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Balance, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_order_book)]
    pub type OrderBook<T: Config> = StorageMap<_, Blake2_128Concat, u32, Vec<u8>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SettingsCreated(Vec<u8>, Vec<u8>), // New settings configuration has been created
        SettingsDestroyed(Vec<u8>),        // A settings has been removed
        KycStored(T::AccountId, Vec<u8>),  // Kyc data stored on chain
        KycSignedforApproval(T::AccountId, T::AccountId), // Kyc has been signed for approval
        KycApproved(T::AccountId, T::AccountId), // Kyc approved with all the required signatures
        IsoCountryCreated(Vec<u8>, Vec<u8>), // Iso country created
        IsoCountryDestroyed(Vec<u8>),      // Iso country destroyed
        CurrencyCodeCreated(Vec<u8>, Vec<u8>), // a currency code has been created
        CurrencyDestroyed(Vec<u8>),        // Currency code has been destroyed
        BondCreated(T::AccountId, u32, Vec<u8>), // New bond has been created
        BondApproved(u32, T::AccountId),   // A bond has been approved
        BondSignedforApproval(u32, T::AccountId), // A bond has been assigned for approval
        CreditRatingAgencyStored(T::AccountId, Vec<u8>), // Credit rating agency has been stored/updated
        CreditRatingStored(u32, Vec<u8>),                // New credit rating has been created
        UnderwriterCreated(T::AccountId, Vec<u8>),       // An underwriter has been created
        UnderwriterDestroyed(T::AccountId),              // An underwriter has been destroyed
        CollateralsStored(u32, u32, Vec<u8>),            // A collaterals has been stored
        CollateralsDestroyed(u32, u32),                  // A collaterals has been destroyed
        CollateralsApproved(u32, u32, Vec<u8>),          // A collaterals has been approved
        FundStored(T::AccountId, Vec<u8>),               // Fund data stored on chain
        FundApproved(T::AccountId, T::AccountId), // Fund approved with all the required signatures
        FundSignedforApproval(T::AccountId, T::AccountId), // Fund has been signed for approval
        InsurerCreated(T::AccountId, Vec<u8>),    // Insurer has been stored/updated
        InsurerDestroyed(T::AccountId),           // Insurer has been destroyed
        InsuranceCreated(T::AccountId, u32, Vec<u8>), // Insurance has been created
        InsuranceDestroyed(T::AccountId, u32),    // Insurance has been destroyed
        InsuranceSigned(T::AccountId, u32, T::AccountId), // Insurance signed
        LawyerCreated(T::AccountId, Vec<u8>),     // A Lawyer has been created
        LawyerDestroyed(T::AccountId),            // A Lawyer opinion has been destroyed
        InterbankRateCreated(Vec<u8>, Vec<u8>),   // An InterbankRate has been created
        InterbankRateDestroyed(Vec<u8>, Vec<u8>), // An InterbankRate has been destroyed
        InflationRateCreated(Vec<u8>, Vec<u8>),   // An Inflation Rate has been created
        InflationRateDestroyed(Vec<u8>, Vec<u8>), // An Inflation Rate has been destroyed
        CreditRatingAgencyDestroyed(T::AccountId), // A credit agency ahs been deleted
        InsuranceFundStaken(T::AccountId, Balance), // Some funds have been stake for Insurer reserve
        InsuranceFundUnstaken(T::AccountId, Balance), // Some funds have been unstaken from Insurer reserve
        BondSubscribed(u32, T::AccountId, u32),       // Subscription of shares of a bond
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Settings key is not valid
        SettingsKeyNotValid,
        /// Settings Key has not been found on the blockchain
        SettingsKeyNotFound,
        /// Settings data is too short to be valid
        SettingsJsonTooShort,
        /// Settings data is too long to be valid
        SettingsJsonTooLong,
        /// Settings key is wrong
        SettingsKeyIsWrong,
        /// Invalid Json structure
        InvalidJson,
        /// Manager account in KYC settings is wrong
        KycManagerAccountIsWrong,
        /// Supervisor account in KYC settings is wrong
        KycSupervisorAccountIsWrong,
        /// Operators account for KYC have not been configured, minimum one account
        KycOperatorsNotConfigured,
        /// Manager account for bond approval is wrong
        BondApprovalManagerAccountIsWrong,
        /// Bond approval commitee is wrong
        BondApprovalCommitteeIsWrong,
        /// Mandatory underwriting can be Y or N only.
        BondApprovalMandatoryUnderwritingIsWrong,
        /// Mandatory credit rating can be Y or N only.
        BondApprovalMandatoryCreditRatingIsWrong,
        /// Mandatory legal opinion can be Y or N only.
        BondApprovalMandatoryLegalOpinionIsWrong,
        /// Manager account for underwriters submission is wrong
        UnderWritersSubmissionManagerAccountIsWrong,
        /// Manager account for lawyers submission is wrong
        LawyersSubmissionManagerAccountIsWrong,
        /// Committe for lawyers submission is wrong
        LawyersSubmissionCommitteeIsWrong,
        /// Manager account for collateral verification is wrong
        CollateralVerificationManagerAccountIsWrong,
        /// Committe for collateral verification is wrong
        CollateralVerificationCommitteeIsWrong,
        ///  Account for  fund approval is wrong
        FundApprovalManagerAccountIsWrong,
        /// Committe for fund approval  is wrong
        FundApprovalCommitteeIsWrong,
        /// Info Documents is empty
        InfoDocumentsIsWrong,
        /// Kyc Id is wrong, it cannot be zero
        KycIdIsWrongCannotBeZero,
        /// Kyc info cannot be longer of 8192 bytes
        KycInfoIsTooLong,
        /// Kyc cannot be shorter of 10 bytes
        KycNameTooShort,
        /// Kyc cannot be longer of 64 bytes
        KycNameTooLong,
        /// Kyc address cannot be shorter of 10 bytes
        KycAddressTooShort,
        /// Kyc address cannot be longer of 64 bytes
        KycAddressTooLong,
        /// Kyc zip code cannot be shorter of 3 bytes
        KycZipCodeTooShort,
        /// Kyc zip code cannot be longer of 6 bytes
        KycZipCodeTooLong,
        /// Kyc city cannot be shorter of 3 bytes
        KycCityTooShort,
        /// Kyc state cannot be longer of 64 bytes
        KycCityTooLong,
        /// Kyc state cannot be shorter of 3 bytes
        KycStateTooShort,
        /// Kyc state cannot be longer of 64 bytes
        KycStateTooLong,
        /// Kyc country cannot be shorter of 3 bytes
        KycCountryTooShort,
        /// Kyc country cannot be longer of 64 bytes
        KycCountryTooLong,
        /// Kyc website is too short
        KycWebSiteTooShort,
        /// Kyc website is too long
        KycWebSiteTooLong,
        /// Kyc website is wrong
        KycWebSiteIsWrong,
        /// Kyc phone is too short
        KycPhoneTooShort,
        /// Kyc phone is too long
        KycPhoneTooLong,
        /// Kyc phone is wrong, not international prefix is matching
        KycPhoneIsWrong,
        /// Document description is too short
        KycDocumentDescriptionTooShort,
        /// Document Ipfs address is too short
        KycDocumentIpfsAddressTooShort,
        /// Missing documents
        KycMissingDocuments,
        /// Kyc Id has not been found
        KycIdNotFound,
        /// The signer has already signed the same kyc
        KycSignatureAlreadyPresentrSameSigner,
        /// Kyc Settings not yet configured
        KycSettingsNotConfigured,
        /// The signer is not authorized to approve a KYC
        SignerIsNotAuthorizedForKycApproval,
        /// The signer is not authorized to delet a KYC
        SignerIsNotAuthorizedForKycCancellation,
        /// Bond id cannot be zero
        BondIdIsWrongCannotBeZero,
        /// Bond id has been already used
        BondIdAlreadyUsed,
        /// Wrong account id
        WrongAccountId,
        /// Missing kyc for the signer
        MissingKycForSigner,
        /// Missing Kyc Approval
        MissingKycApproval,
        /// Bond total amount cannot be zero
        BondTotalAmountCannotBeZero,
        /// Country code lenght is wrong
        WrongLengthCountryCode,
        /// Country name is too short
        CountryNameTooShort,
        /// Country code is already present
        CountryCodeAlreadyPresent,
        /// Country code not found
        CountryCodeNotFound,
        /// Wrong lenght of the currency code
        WrongLengthCurrencyCode,
        /// info field is too long
        SizeInfoTooLong,
        /// Currency name is too short
        CurrencyNameTooShort,
        /// Currency name is too long
        CurrencyNameTooLong,
        /// Currency category is wrong, it can be 'c' for crypto currency or 'f' for fiat currency
        CurrencyCategoryIswrong,
        /// Currency code has not been found
        CurrencyCodeNotFound,
        /// block chain name is too short
        BlockchainNameTooShort,
        /// block chain name is too long
        BlockchainNameTooLong,
        /// Currency code is already present
        CurrencyCodeAlreadyPresent,
        /// Bond interest rate cannot be zero
        BondInterestRateIsWrong,
        /// Bond interest type is wrong, it can be X,F,Z,I only.
        BondInterestTypeIsWrong,
        /// Bond maturity cannot be zero
        BondMaturityCannotBeZero,
        /// Bond maturity cannot be longer than 600 months (50 years)
        BondMaturityTooLong,
        /// Bond Instalments cannot be more than 600
        BondTooManyInstalments,
        /// Bond instalmnents cannot exceed Maturity
        BondInstalmentsCannotExceedMaturity,
        /// Grace period cannot exceed the maturity
        BondGracePeriodCannotExceedMaturity,
        /// Bond accepted currencies cannot be empty
        BondAcceptedCurrenciesCannotBeEmpty,
        /// Bond subordinated is wrong, it can be Y/N
        BondSubordinatedIsWrong,
        /// Bond put option is wrong, it can be Y/N
        BondPutOptionIsWrong,
        /// Bond put vesting period cannot be zero
        BondPutVestingPeriodCannotBeZero,
        /// Bond call option is wrong, it can be Y/N
        BondCallOptionIsWrong,
        /// Bond put vesting period cannot be zero
        BondCallVestingPeriodCannotBeZero,
        /// Bond put convertible option is wrong, it can be Y/N
        BondPutConvertibleOptionIsWrong,
        /// Bond call convertible option is wrong, it can be Y/N
        BondCallConvertibleOptionIsWrong,
        // The bond document description cannot be shorter of 5 bytes
        BondDocumentDescriptionTooShort,
        /// The ipfs address of the bond document is too short to be real
        BondDocumentIpfsAddressTooShort,
        /// Bond documents are missing
        BondMissingDocuments,
        /// Kyc of the signer is not present or not approved yet
        KycSignerIsNotApproved,
        /// Kyc is under process it cannot be changed
        KycUnderProcessItCannotBeChanged,
        /// Bonds Id has not been found on chain
        BondsIdNotFound,
        /// The signature for the bond approval is alredy present for the same signer
        BondsSignatureAlreadyPresentrSameSigner,
        /// Signer is not authorized for Bond approval
        SignerIsNotAuthorizedForBondApproval,
        /// The manager enabled to submit a credit rating agency is wrong
        CreditRatingAgenciesSubmissionManagerAccountIsWrong,
        /// The committee enabled to submit a credit rating agency is wrong
        CreditRatingAgenciesSubmissionCommitteeIsWrong,
        /// The signed is not authorized to submit a credit rating agency
        SignerIsNotAuthorizedForCreditRatingAgencySubmission,
        /// the json structure for credit rating agency is too long
        CreditRatingAgencyInfoIsTooLong,
        /// The name of the credit rating agency is too short
        CreditRatingAgencyNameTooShort,
        /// The name of the credit rating agency is too long
        CreditRatingAgencyNameTooLong,
        /// The account id of the credit rating agency is wrong, it shoulf be 48 bytes
        CreditRatingAgencyAccountIdIsWrong,
        /// The website of the Credit Rating Agency is wrong
        CreditRatingAgencyWebSiteIsWrong,
        /// Description of the document for the credit agency is too short
        CreditRatingAgencyDocumentDescriptionTooShort,
        /// IPFS address of the document for a credit agency is too short
        CreditRatingAgencyDocumentIpfsAddressTooShort,
        /// Documents for the credit rating agency are missing, at the least one is required
        CreditRatingAgencyMissingDocuments,
        /// The signer is not a credit rating agency
        SignerIsNotAuthorizedAsCreditRatingAgency,
        /// The credit rating info is too long, maximum 8192
        CreditRatingInfoIsTooLong,
        /// The credit rating description is too short
        CreditRatingDescriptionTooShort,
        /// The credit rating description is too long
        CreditRatingDescriptionTooLong,
        /// the credit rating document description is too long
        CreditRatingDocumentDescriptionTooShort,
        /// IPFS address of the document of credit rating is too short
        CreditRatingDocumentIpfsAddressTooShort,
        /// Documents for the credit rating  are missing, at the least one is required
        CreditRatingMissingDocuments,
        /// Underwriter account is not found on chain
        UnderwriterAccountNotFound,
        /// Underwriter account already stored on chain
        UnderwriterAlreadyPresent,
        /// The collaterals info are too long, maximum 8192 bytes
        CollateralsInfoIsTooLong,
        /// Collateral document description is too short
        CollateralDocumentDescriptionTooShort,
        /// Collateral document ipfs address is too short
        CollateralDocumentIpfsAddressTooShort,
        /// Collateral requires at least one document
        CollateralMissingDocuments,
        /// The collateral id is already present per this bond id
        CollateralIdAlreadyPresent,
        /// The signer cannot approve a collateral
        SignerIsNotAuthorizedForCollateralsApproval,
        /// Underwriter InfoDocuments Missing
        MissingUnderwriterInfoDocuments,
        /// Underwriter name is too short
        UnderwriterNameTooShort,
        /// Underwriter website is too short
        UnderwriterWebSiteTooShort,
        /// Underwriter website is too long
        UnderwriterWebSiteTooLong,
        /// Invalid Website
        InvalidWebsite,
        /// The adrresses for the underwriter from json and passed paramenters did not match
        UnmatchingUnderwriterAddress,
        /// Missing Underwriter Account ID from json input
        MissingUnderwriterAccountId,
        /// The committee enabled to submit Underwriter is wrong
        UnderwritersSubmissionCommitteeIsWrong,
        /// The signed is not authorized to submit or remove an underwriter
        SignerIsNotAuthorizedForUnderwriterSubmissionOrRemoval,
        /// Signer is not authorized for fund creation/update
        SignerIsNotAuthorizedForFundCreation,
        /// Insurer manager is not set
        InsurerSubmissionManagerAccountIsWrong,
        /// Insurer committee is empty
        InsurerSubmissionCommitteeIsWrong,
        /// Signer has not authorization to submite Insurer
        SignerIsNotAuthorizedForInsurerSubmissionOrRemoval,
        /// Fund info is long, maximum 8192 bytes
        FundInfoIsTooLong,
        /// Fund name is too short
        FundNameTooShort,
        /// Fund name is too long
        FundNameTooLong,
        /// Fund address cannot be shorter of 10 bytes
        FundAddressTooShort,
        /// Fund address cannot be longer of 64 bytes
        FundAddressTooLong,
        /// Fund zip code cannot be shorter of 3 bytes
        FundZipCodeTooShort,
        /// Fund zip code cannot be longer of 6 bytes
        FundZipCodeTooLong,
        /// Fund city cannot be shorter of 3 bytes
        FundCityTooShort,
        /// Fund city cannot be longer of 64 bytes
        FundCityTooLong,
        /// Fund state cannot be shorter of 3 bytes
        FundStateTooShort,
        /// Fund state cannot be longer of 64 bytes
        FundStateTooLong,
        /// Fund country cannot be shorter of 3 bytes
        FundCountryTooShort,
        /// Fund country cannot be longer of 64 bytes
        FundCountryTooLong,
        /// Fund website is too short
        FundWebSiteTooShort,
        /// Fund website is too long
        FundWebSiteTooLong,
        /// Fund website is wrong
        FundWebSiteIsWrong,
        /// Fund phone is too short
        FundPhoneTooShort,
        /// Fund phone is too long
        FundPhoneTooLong,
        /// Fund phone is wrong, not international prefix is matching
        FundPhoneIsWrong,
        /// Document description is too short for the fund submission
        FundDocumentDescriptionTooShort,
        /// Document Ipfs address is too short
        FundDocumentIpfsAddressTooShort,
        /// Missing documents for the fund
        FundMissingDocuments,
        /// Fund initial fee cannot be zero
        FundInitialFeesCannotBeZero,
        /// Fund yearly fees cannot be zero
        FundYearlyFeesCannotBeZero,
        /// Fund performance fees cannot be zero
        FundPerformanceFeesCannotBeZero,
        /// Fund account id is wrong
        FundAccountIdIsWrong,
        /// Account id for deposit to the fund is wrong
        FundDepositAccountIdIsWrong,
        /// Account id for fund manager is wrong
        FundManagerAccountIdIsWrong,
        /// Fund manager account is missing
        FundManagerAccountisMissing,
        /// Fund under processing cannot be changed
        FundUnderProcessItCannotBeChanged,
        /// Fund signatures is already present on chain for the same signer
        FundsSignatureAlreadyPresentrSameSigner,
        /// Fund type is wrong it can be H for Hedge Fund or E for Enterprise fund
        FundTypeIsWrong,
        /// Signer is not authorized for fund approval
        SignerIsNotAuthorizedForFundApproval,
        /// Currency in the Insurance configuration is wrong
        InsuranceCurrencyIsWrong,
        /// Insurance Minimum Reserver Cannot Be Zero
        InsuranceMinReserveCannotBeZero,
        /// Insurer Settings is missing from the configuration
        InsurerSettingIsMissing,
        /// Insurer is already present
        InsurerAlreadyPresent,
        /// Insurer name is too short
        InsurerNameTooShort,
        /// Insurer web site is too short
        InsurerWebSiteTooShort,
        /// Insurer web site is too long
        InsurerWebSiteTooLong,
        /// The adrresses for the underwriter from json and passed paramenters did not match
        UnmatchingInsurerAddress,
        /// Insurer account not found
        InsurerAccountNotFound,
        /// Insurer account has not been found
        MissingInsurerAccountId,
        /// Info documents for the insurer are missing
        MissingInsurerInfoDocuments,
        /// Signer is not an insurer
        SignerIsNotInsurer,
        /// Max Coverage cannot be zero
        MaxCoverageCannotBeZero,
        /// Payer account is wrong
        PayerAccountIsWrong,
        /// Beneficiary account is wrong
        BeneficiaryAccountIsWrong,
        /// Insurance premium cannot be zero.
        InsurancePremiumCannotBeZero,
        /// Information documents about the insurance are missing
        MissingInsuranceInfoDocuments,
        /// Insurance id is already present in the state
        InsuranceIdAlreadyPresent,
        /// The data is already stored on chain
        AlreadyPresent,
        /// The name is too short
        NameTooShort,
        /// The website is too short
        WebSiteTooShort,
        /// The website is too short
        WebSiteTooLong,
        /// The account id is missing in the parameters passed
        MissingAccountId,
        /// The addresses did not match
        UnmatchingAddress,
        /// The info documents is missing
        MissingInfoDocuments,
        /// Signer Is Not Authorized For Submission Or Removal
        SignerIsNotAuthorizedForSubmissionOrRemoval,
        /// Account if has not  been found
        AccountNotFound,
        ///Current insurer reserves below minimum insurer requirement
        BelowMinimumReserve,
        /// The current reserve is not enough for the additional insurance
        InsufficientReserve,
        /// Insurance cannot be found
        InsuranceNotFound,
        ///Insurer reserve not found
        ReserveNotFound,
        /// Insurance has been already signed
        InsuranceAlreadySigned,
        /// The date format is not in  YYYY-MM-DD format
        InvalidDateFormat,
        /// Settings does not exist
        SettingsDoesNotExist,
        /// Got an overflow after adding
        Overflow,
        /// Underflow after substrate
        Underflow,
        /// The signer is not owning any approved Fund
        FundNotFoundforSigner,
        /// The fund owned from the signer is not yet approved
        FundNotYetApproved,
        /// Signer is not matching the Owner account in the json structure.
        SignerIsNotMatchingOwnerAccount,
        /// The owner account field is mandatory, it must match the signer of the transaction
        OwnerAccountIsMissing,
        /// Bond is under approval it cannot be changed more
        BondUnderApprovalCannotBeChanged,
        /// Bond has been already approved
        BondAlreadyApproved,
        /// Only the manager can delete a credit rating agency.
        SignerIsNotAuthorizedForCreditRatingAgencyCancellation,
        /// Credit Rating Agency has not been found
        CreditRatingAgencyNotFound,
        /// Credit rating documents are mandatory
        CreditRatingDocumentsAreMissing,
        /// Collateral Id cannot be found on chain
        CollateralIdNotFound,
        /// Collater has been already approved
        CollateralAlreadyApproved,
        /// Underwriter document description is too short
        UnderwriterDocumentDescriptionTooShort,
        /// Underwriter document ipfs address is too short
        UnderwriterDocumentIpfsAddressTooShort,
        /// Underwriter requires at least one document
        UnderwriterMissingDocuments,
        /// Insurer document description is too short
        InsurerDocumentDescriptionTooShort,
        /// Insurer document ipfs address is too short
        InsurerDocumentIpfsAddressTooShort,
        /// Insurer requires at least one document
        InsurerMissingDocuments,
        /// Lawyer document description is too short
        LawyerDocumentDescriptionTooShort,
        /// Lawyer document ipfs address is too short
        LawyerDocumentIpfsAddressTooShort,
        /// Lawyer requires at least one document
        LawyerMissingDocuments,
        /// Interbank rate is already stored for the same date of reference.
        InterbankRateAlreadyPresent,
        /// Inflation rate is already stored for the same date of reference.
        InflationRateAlreadyPresent,
        /// Insurance reserve account is wrong
        InsuranceReserveAccountIswrong,
        /// Insurance reserve account is not set
        InsuranceReserveAccountNotSet,
        /// Current reserve is not enough
        CurrentReserveIsNotEnough,
        /// Insufficient funds available for the payment
        InsufficientFunds,
        /// Currency code has not a valid address in the blockchain
        CurrencyCodeHasNotValidAddress,
        ///Default Token id cannot be zero
        TokenidCannotBeZero,
        /// Total shares available are not enough
        TotalShareAvailablesNotEnough,
        // Stable coin has not been configured
        MissingStableCoinConfiguration,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create/change a  settings configuration. Reserved to super user
        /// We have multiple of configuration:
        /// key=="kyc" {"manager":"xxxaccountidxxx","supervisor":"xxxxaccountidxxxx","operators":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}
        /// for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","supervisor":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","operators":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]}
        /// key=="bondapproval" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...],"mandatoryunderwriting":"Y/N","mandatorycreditrating":"Y/N","mandatorylegalopinion":"Y/N"}
        /// for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"],"mandatoryunderwriting":"Y","mandatorycreditrating":"Y","mandatorylegalopinion":"Y"}
        /// key=="underwriterssubmission" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}
        /// for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}
        /// key=="infodocuments" {"documents:[{"document":"xxxxdescription"},{"document":"xxxxdescription"}]}
        /// for example: [{"document":"Profit&Loss Previous year"},{"document":"Board Members/Director List"}]
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn create_change_settings(
            origin: OriginFor<T>,
            key: Vec<u8>,
            configuration: Vec<u8>,
        ) -> DispatchResult {
            // check the request is signed from Super User
            let _sender = ensure_root(origin)?;
            //check configuration length
            ensure!(configuration.len() > 12, Error::<T>::SettingsJsonTooShort);
            ensure!(configuration.len() < 8192, Error::<T>::SettingsJsonTooLong);
            // check json validity
            let js = configuration.clone();
            ensure!(json_check_validity(js), Error::<T>::InvalidJson);
            // check for key validity
            ensure!(
                key == "kyc".as_bytes().to_vec()
                    || key == "bondapproval".as_bytes().to_vec()
                    || key == "underwriterssubmission".as_bytes().to_vec()
                    || key == "insurerssubmission".as_bytes().to_vec()
                    || key == "creditratingagencies".as_bytes().to_vec()
                    || key == "collateralsverification".as_bytes().to_vec()
                    || key == "fundapproval".as_bytes().to_vec()
                    || key == "lawyerssubmission".as_bytes().to_vec()
                    || key == "infodocuments".as_bytes().to_vec()
                    || key == "insuranceminreserve".as_bytes().to_vec()
                    || key == "stablecoin".as_bytes().to_vec(),
                Error::<T>::SettingsKeyIsWrong
            );
            // check validity for kyc settings
            if key == "kyc".as_bytes().to_vec() {
                let manager = json_get_value(configuration.clone(), "manager".as_bytes().to_vec());
                ensure!(
                    manager.len() == 48 || manager.is_empty(),
                    Error::<T>::KycManagerAccountIsWrong
                );
                let supervisor =
                    json_get_value(configuration.clone(), "supervisor".as_bytes().to_vec());
                ensure!(
                    supervisor.len() == 48 || supervisor.is_empty(),
                    Error::<T>::KycSupervisorAccountIsWrong
                );
                let operators =
                    json_get_complexarray(configuration.clone(), "operators".as_bytes().to_vec());
                if operators.len() >= 2 {
                    let mut x = 0;
                    loop {
                        let w = json_get_recordvalue(operators.clone(), x);
                        if w.is_empty() {
                            break;
                        }
                        x += 1;
                    }
                    // at the least one operator is required for this configuration.
                    ensure!(x > 0, Error::<T>::KycOperatorsNotConfigured);
                }
            }
            // check validity for bond approval settings
            if key == "bondapproval".as_bytes().to_vec() {
                let manager = json_get_value(configuration.clone(), "manager".as_bytes().to_vec());
                ensure!(
                    manager.len() == 48 || manager.is_empty(),
                    Error::<T>::BondApprovalManagerAccountIsWrong
                );
                let committee =
                    json_get_complexarray(configuration.clone(), "committee".as_bytes().to_vec());
                let mut x = 0;
                if committee.len() > 2 {
                    loop {
                        let w = json_get_recordvalue(committee.clone(), x);
                        if w.is_empty() {
                            break;
                        }
                        x += 1;
                    }
                }
                ensure!(x > 0, Error::<T>::BondApprovalCommitteeIsWrong);
                let mandatoryunderwriting = json_get_value(
                    configuration.clone(),
                    "mandatoryunderwriting".as_bytes().to_vec(),
                );
                ensure!(
                    mandatoryunderwriting == "Y".as_bytes().to_vec()
                        || mandatoryunderwriting == "N".as_bytes().to_vec(),
                    Error::<T>::BondApprovalMandatoryUnderwritingIsWrong
                );
                let mandatorycreditrating = json_get_value(
                    configuration.clone(),
                    "mandatorycreditrating".as_bytes().to_vec(),
                );
                ensure!(
                    mandatorycreditrating == "Y".as_bytes().to_vec()
                        || mandatorycreditrating == "N".as_bytes().to_vec(),
                    Error::<T>::BondApprovalMandatoryCreditRatingIsWrong
                );
                let mandatorylegalopinion = json_get_value(
                    configuration.clone(),
                    "mandatorylegalopinion".as_bytes().to_vec(),
                );
                ensure!(
                    mandatorylegalopinion == "Y".as_bytes().to_vec()
                        || mandatorylegalopinion == "N".as_bytes().to_vec(),
                    Error::<T>::BondApprovalMandatoryLegalOpinionIsWrong
                );
            }
            // check validity for Under Writers submission settings
            if key == "underwriterssubmission".as_bytes().to_vec() {
                let manager = json_get_value(configuration.clone(), "manager".as_bytes().to_vec());
                ensure!(
                    manager.len() == 48 || manager.is_empty(),
                    Error::<T>::UnderWritersSubmissionManagerAccountIsWrong
                );
                let committee =
                    json_get_complexarray(configuration.clone(), "committee".as_bytes().to_vec());
                let mut x = 0;
                if committee.len() > 2 {
                    loop {
                        let w = json_get_recordvalue(committee.clone(), x);
                        if w.is_empty() {
                            break;
                        }
                        x += 1;
                    }
                }
                ensure!(x > 0, Error::<T>::UnderwritersSubmissionCommitteeIsWrong);
            }
            // check validity for insurer submission settings
            if key == "insurerssubmission".as_bytes().to_vec() {
                let manager = json_get_value(configuration.clone(), "manager".as_bytes().to_vec());
                ensure!(
                    manager.len() == 48 || manager.is_empty(),
                    Error::<T>::InsurerSubmissionManagerAccountIsWrong
                );
                let committee =
                    json_get_complexarray(configuration.clone(), "committee".as_bytes().to_vec());
                let mut x = 0;
                if committee.len() > 2 {
                    loop {
                        let w = json_get_recordvalue(committee.clone(), x);
                        if w.is_empty() {
                            break;
                        }
                        x += 1;
                    }
                }
                ensure!(x > 0, Error::<T>::InsurerSubmissionCommitteeIsWrong);
            }
            // check validity for insurance reserve account
            if key == "insurancereserve".as_bytes().to_vec() {
                let account = json_get_value(configuration.clone(), "account".as_bytes().to_vec());
                ensure!(
                    account.len() == 48 || account.is_empty(),
                    Error::<T>::InsuranceReserveAccountIswrong
                );
            }
            // check validity for the submission settings of credit rating agencies
            if key == "creditratingagencies".as_bytes().to_vec() {
                let manager = json_get_value(configuration.clone(), "manager".as_bytes().to_vec());
                ensure!(
                    manager.len() == 48 || manager.is_empty(),
                    Error::<T>::CreditRatingAgenciesSubmissionManagerAccountIsWrong
                );
                let committee =
                    json_get_complexarray(configuration.clone(), "committee".as_bytes().to_vec());
                let mut x = 0;
                if committee.len() > 2 {
                    loop {
                        let w = json_get_recordvalue(committee.clone(), x);
                        if w.is_empty() {
                            break;
                        }
                        x += 1;
                    }
                }
                ensure!(
                    x > 0,
                    Error::<T>::CreditRatingAgenciesSubmissionCommitteeIsWrong
                );
            }
            // check validity for lawyers submission settings
            if key == "lawyerssubmission".as_bytes().to_vec() {
                let manager = json_get_value(configuration.clone(), "manager".as_bytes().to_vec());
                ensure!(
                    manager.len() == 48 || manager.is_empty(),
                    Error::<T>::LawyersSubmissionManagerAccountIsWrong
                );
                let committee =
                    json_get_complexarray(configuration.clone(), "committee".as_bytes().to_vec());
                let mut x = 0;
                if committee.len() > 2 {
                    loop {
                        let w = json_get_recordvalue(committee.clone(), x);
                        if w.is_empty() {
                            break;
                        }
                        x += 1;
                    }
                }
                ensure!(x > 0, Error::<T>::LawyersSubmissionCommitteeIsWrong);
            }
            // check validity for collateral verification settings
            if key == "collateralsverification".as_bytes().to_vec() {
                let manager = json_get_value(configuration.clone(), "manager".as_bytes().to_vec());
                ensure!(
                    manager.len() == 48 || manager.is_empty(),
                    Error::<T>::CollateralVerificationManagerAccountIsWrong
                );
                let committee =
                    json_get_complexarray(configuration.clone(), "committee".as_bytes().to_vec());
                let mut x = 0;
                if committee.len() > 2 {
                    loop {
                        let w = json_get_recordvalue(committee.clone(), x);
                        if w.is_empty() {
                            break;
                        }
                        x += 1;
                    }
                }
                ensure!(x > 0, Error::<T>::CollateralVerificationCommitteeIsWrong);
            }
            // check validity for enterprise/edge fund approval settings
            if key == "fundapproval".as_bytes().to_vec() {
                let manager = json_get_value(configuration.clone(), "manager".as_bytes().to_vec());
                ensure!(
                    manager.len() == 48 || manager.is_empty(),
                    Error::<T>::FundApprovalManagerAccountIsWrong
                );
                let committee =
                    json_get_complexarray(configuration.clone(), "committee".as_bytes().to_vec());
                let mut x = 0;
                if committee.len() > 2 {
                    loop {
                        let w = json_get_recordvalue(committee.clone(), x);
                        if w.is_empty() {
                            break;
                        }
                        x += 1;
                    }
                }
                ensure!(x > 0, Error::<T>::FundApprovalCommitteeIsWrong);
            }
            // check validity for stable coin settings
            if key == "stablecoin".as_bytes().to_vec() {
                let tokenid = json_get_value(configuration.clone(), "tokenid".as_bytes().to_vec());
                let tokenidv = vecu8_to_u32(tokenid);
                ensure!(tokenidv > 0, Error::<T>::TokenidCannotBeZero);
            }
            // check validity for info documents, with this option you can configure the mandatory documents required when creating a bond.
            if key == "infodocuments".as_bytes().to_vec() {
                let documents =
                    json_get_complexarray(configuration.clone(), "documents".as_bytes().to_vec());
                let mut x = 0;
                if documents.len() > 2 {
                    loop {
                        let w = json_get_recordvalue(documents.clone(), x);
                        if w.is_empty() {
                            break;
                        }
                        x += 1;
                    }
                }
                ensure!(x > 0, Error::<T>::InfoDocumentsIsWrong);
            }
            // check validity for collateral verification settings
            if key == "insuranceminreserve".as_bytes().to_vec() {
                let currency =
                    json_get_value(configuration.clone(), "currency".as_bytes().to_vec());
                ensure!(currency.len() >= 3, Error::<T>::InsuranceCurrencyIsWrong);
                let reserve = json_get_value(configuration.clone(), "reserve".as_bytes().to_vec());
                let reservev = vecu8_to_u32(reserve);
                ensure!(reservev > 0, Error::<T>::InsuranceMinReserveCannotBeZero);
            }
            //store settings on chain
            if Settings::<T>::contains_key(&key) {
                // Replace Settings Data
                Settings::<T>::take(key.clone());
            }
            Settings::<T>::insert(key.clone(), configuration.clone());
            // Generate event
            Self::deposit_event(Event::SettingsCreated(key, configuration));
            // Return a successful DispatchResult
            Ok(())
        }

        // this function has the purpose to the insert or update data for KYC
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn create_change_kyc(
            origin: OriginFor<T>,
            accountid: T::AccountId,
            info: Vec<u8>,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // check the signer is one of the operators for kyc
            ensure!(
                Settings::<T>::contains_key("kyc".as_bytes().to_vec()),
                Error::<T>::KycSettingsNotConfigured
            );
            let json: Vec<u8> = Settings::<T>::get("kyc".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            let mut signingtype = 0;
            // check the signer is  the manager for kyc
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                    signingtype = 1;
                }
            }
            // check the signer is is the supervisor for kyc
            let supervisor = json_get_value(json.clone(), "supervisor".as_bytes().to_vec());
            if !supervisor.is_empty() {
                let supervisorvec = bs58::decode(supervisor).into_vec().unwrap();
                let accountidsupervisor = T::AccountId::decode(&mut &supervisorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidsupervisor {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 2;
                    }
                }
            }
            // check the signer is one of the operators for kyc
            let operators = json_get_complexarray(json, "operators".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let operator = json_get_arrayvalue(operators.clone(), x);
                if operator.is_empty() {
                    break;
                }
                let operatorvec = bs58::decode(operator).into_vec().unwrap();
                let accountidoperator = T::AccountId::decode(&mut &operatorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if accountidoperator == signer {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 3;
                    }
                }
                x += 1;
            }
            ensure!(flag == 1, Error::<T>::SignerIsNotAuthorizedForKycApproval);
            //check info length
            ensure!(info.len() < 8192, Error::<T>::KycInfoIsTooLong);
            // check json validity
            let js = info.clone();
            ensure!(json_check_validity(js), Error::<T>::InvalidJson);
            // check name
            let name = json_get_value(info.clone(), "name".as_bytes().to_vec());
            ensure!(name.len() >= 3, Error::<T>::KycNameTooShort);
            ensure!(name.len() <= 64, Error::<T>::KycNameTooLong);
            // check Address
            let address = json_get_value(info.clone(), "address".as_bytes().to_vec());
            ensure!(address.len() >= 3, Error::<T>::KycAddressTooShort);
            ensure!(address.len() <= 64, Error::<T>::KycAddressTooLong);
            // check Zip code
            let zip = json_get_value(info.clone(), "zip".as_bytes().to_vec());
            ensure!(zip.len() > 3, Error::<T>::KycZipCodeTooShort);
            ensure!(zip.len() <= 6, Error::<T>::KycZipCodeTooLong);
            // check City
            let city = json_get_value(info.clone(), "city".as_bytes().to_vec());
            ensure!(city.len() > 3, Error::<T>::KycCityTooShort);
            ensure!(city.len() <= 64, Error::<T>::KycCityTooLong);
            // check State
            let state = json_get_value(info.clone(), "state".as_bytes().to_vec());
            ensure!(state.len() > 3, Error::<T>::KycStateTooShort);
            ensure!(state.len() <= 64, Error::<T>::KycStateTooLong);
            // check Country
            let country = json_get_value(info.clone(), "country".as_bytes().to_vec());
            ensure!(country.len() > 2, Error::<T>::KycCountryTooShort);
            ensure!(country.len() < 64, Error::<T>::KycCountryTooLong);
            // check Website
            let website = json_get_value(info.clone(), "website".as_bytes().to_vec());
            ensure!(website.len() >= 5, Error::<T>::KycWebSiteTooShort);
            ensure!(website.len() <= 64, Error::<T>::KycWebSiteTooLong);
            ensure!(validate_weburl(website), Error::<T>::KycWebSiteIsWrong);
            // check Phone
            let phone = json_get_value(info.clone(), "phone".as_bytes().to_vec());
            ensure!(phone.len() >= 10, Error::<T>::KycPhoneTooShort);
            ensure!(phone.len() <= 21, Error::<T>::KycPhoneTooLong);
            ensure!(validate_phonenumber(phone), Error::<T>::KycPhoneIsWrong);
            let ipfsdocs = json_get_complexarray(info.clone(), "ipfsdocs".as_bytes().to_vec());
            if ipfsdocs.len() > 2 {
                let mut x = 0;
                loop {
                    let w = json_get_recordvalue(ipfsdocs.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    let description = json_get_value(w.clone(), "description".as_bytes().to_vec());
                    ensure!(
                        description.len() > 5,
                        Error::<T>::KycDocumentDescriptionTooShort
                    );
                    let ipfsaddress = json_get_value(w.clone(), "ipfsaddress".as_bytes().to_vec());
                    ensure!(
                        ipfsaddress.len() > 20,
                        Error::<T>::KycDocumentIpfsAddressTooShort
                    );

                    x += 1;
                }
                ensure!(x > 0, Error::<T>::KycMissingDocuments);
            }
            //store Kyc on chain subject to further approval
            if Kyc::<T>::contains_key(&accountid) {
                // check that is not already approved from anybody
                let itr = KycSignatures::<T>::iter_prefix(accountid.clone());
                ensure!(
                    itr.count() == 0,
                    Error::<T>::KycUnderProcessItCannotBeChanged
                );
                // Replace Kyc Data
                Kyc::<T>::take(accountid.clone());
            }
            Kyc::<T>::insert(accountid.clone(), info.clone());
            // Generate event
            Self::deposit_event(Event::KycStored(accountid, info));
            // Return a successful DispatchResult
            Ok(())
        }
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn kyc_approve(origin: OriginFor<T>, accountid: T::AccountId) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            let mut signingtype = 0;
            //check id >0
            ensure!(
                Kyc::<T>::contains_key(&accountid),
                Error::<T>::KycIdNotFound
            );
            ensure!(
                !KycSignatures::<T>::contains_key(&accountid, &signer),
                Error::<T>::KycSignatureAlreadyPresentrSameSigner
            );
            // check the signer is one of the supervisors or manager for kyc
            let json: Vec<u8> = Settings::<T>::get("kyc".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                    signingtype = 1;
                }
            }
            // signed from supervisor
            let supervisor = json_get_value(json.clone(), "supervisor".as_bytes().to_vec());
            if !supervisor.is_empty() {
                let supervisorvec = bs58::decode(supervisor).into_vec().unwrap();
                let accountidsupervisor = T::AccountId::decode(&mut &supervisorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidsupervisor {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 2;
                    }
                }
            }
            let operators = json_get_complexarray(json, "operators".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let operator = json_get_arrayvalue(operators.clone(), x);
                if operator.is_empty() {
                    break;
                }
                let operatorvec = bs58::decode(operator).into_vec().unwrap();
                let accountidoperator = T::AccountId::decode(&mut &operatorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if accountidoperator == signer {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 3;
                    }
                }
                x += 1;
            }
            ensure!(flag == 1, Error::<T>::SignerIsNotAuthorizedForKycApproval);
            // write/update signature
            KycSignatures::<T>::insert(accountid.clone(), signer.clone(), signingtype);
            // check for all the approval
            let mut sigmanager = 0;
            let mut sigsupervisor = 0;
            let mut itr = KycSignatures::<T>::iter_prefix(accountid.clone());
            let mut result;
            loop {
                result = itr.next();
                match result {
                    Some(x) => {
                        if x.1 == 1 {
                            sigmanager = 1;
                        }
                        if x.1 == 2 {
                            sigsupervisor = 1;
                        }
                    }
                    None => break,
                }
            }
            // store approved flag if all signatures have been received
            if sigmanager == 1 && sigsupervisor == 1 {
                KycApproved::<T>::insert(accountid.clone(), 1);
                // generate event for approved
                Self::deposit_event(Event::KycApproved(accountid.clone(), signer.clone()));
            }
            // generate event for the approval
            Self::deposit_event(Event::KycSignedforApproval(accountid, signer));
            // Return a successful DispatchResult
            Ok(())
        }
        // function to delete a KYC data set, it can be executed only frm the manager or supervisor of KYC till the KYC is not yet fully approved.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn kyc_delete(origin: OriginFor<T>, accountid: T::AccountId) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            //check id >0
            ensure!(
                Kyc::<T>::contains_key(&accountid),
                Error::<T>::KycIdNotFound
            );
            // check the signer is one of the supervisors or manager for kyc
            let json: Vec<u8> = Settings::<T>::get("kyc".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                }
            }
            // signed from supervisor
            let supervisor = json_get_value(json.clone(), "supervisor".as_bytes().to_vec());
            if !supervisor.is_empty() {
                let supervisorvec = bs58::decode(supervisor).into_vec().unwrap();
                let accountidsupervisor = T::AccountId::decode(&mut &supervisorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidsupervisor {
                    flag = 1;
                }
            }
            ensure!(
                flag == 1,
                Error::<T>::SignerIsNotAuthorizedForKycCancellation
            );
            // delete the KYC
            Kyc::<T>::take(&accountid).unwrap();
            // generate event for the cancellation
            Self::deposit_event(Event::KycSignedforApproval(accountid, signer));
            // Return a successful DispatchResult
            Ok(())
        }
        // this function has the purpose to the insert or update data for a Fund (hedge or enterprise)
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn fund_create_change(
            origin: OriginFor<T>,
            accountid: T::AccountId,
            info: Vec<u8>,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // check the signer is one of the operators for kyc
            ensure!(
                Settings::<T>::contains_key("kyc".as_bytes().to_vec()),
                Error::<T>::SettingsDoesNotExist
            );
            let json: Vec<u8> = Settings::<T>::get("kyc".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            let mut signingtype = 0;
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                    signingtype = 1;
                }
            }
            let supervisor = json_get_value(json.clone(), "supervisor".as_bytes().to_vec());
            if !supervisor.is_empty() {
                let supervisorvec = bs58::decode(supervisor).into_vec().unwrap();
                let accountidsupervisor = T::AccountId::decode(&mut &supervisorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidsupervisor {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 2;
                    }
                }
            }
            let operators = json_get_complexarray(json, "operators".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let operator = json_get_arrayvalue(operators.clone(), x);
                if operator.is_empty() {
                    break;
                }
                let operatorvec = bs58::decode(operator).into_vec().unwrap();
                let accountidoperator = T::AccountId::decode(&mut &operatorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if accountidoperator == signer {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 3;
                    }
                }
                x += 1;
            }
            ensure!(flag == 1, Error::<T>::SignerIsNotAuthorizedForFundCreation);
            //check info length
            ensure!(info.len() < 8192, Error::<T>::FundInfoIsTooLong);
            // check json validity
            let js = info.clone();
            ensure!(json_check_validity(js), Error::<T>::InvalidJson);
            // check name
            let name = json_get_value(info.clone(), "name".as_bytes().to_vec());
            ensure!(name.len() >= 10, Error::<T>::FundNameTooShort);
            ensure!(name.len() <= 64, Error::<T>::FundNameTooLong);
            // check Address
            let address = json_get_value(info.clone(), "address".as_bytes().to_vec());
            ensure!(address.len() >= 10, Error::<T>::FundAddressTooShort);
            ensure!(address.len() <= 64, Error::<T>::FundAddressTooLong);
            // check Zip code
            let zip = json_get_value(info.clone(), "zip".as_bytes().to_vec());
            ensure!(zip.len() > 3, Error::<T>::FundZipCodeTooShort);
            ensure!(zip.len() <= 6, Error::<T>::FundZipCodeTooLong);
            // check City
            let city = json_get_value(info.clone(), "city".as_bytes().to_vec());
            ensure!(city.len() > 3, Error::<T>::FundCityTooShort);
            ensure!(city.len() <= 64, Error::<T>::FundCityTooLong);
            // check State
            let state = json_get_value(info.clone(), "state".as_bytes().to_vec());
            ensure!(state.len() > 3, Error::<T>::FundStateTooShort);
            ensure!(state.len() <= 64, Error::<T>::FundStateTooLong);
            // check Country
            let country = json_get_value(info.clone(), "country".as_bytes().to_vec());
            ensure!(country.len() > 3, Error::<T>::FundCountryTooShort);
            ensure!(country.len() < 64, Error::<T>::FundCountryTooLong);
            // check Website
            let website = json_get_value(info.clone(), "website".as_bytes().to_vec());
            ensure!(website.len() >= 10, Error::<T>::FundWebSiteTooShort);
            ensure!(website.len() <= 64, Error::<T>::FundWebSiteTooLong);
            ensure!(validate_weburl(website), Error::<T>::FundWebSiteIsWrong);
            // check Phone
            let phone = json_get_value(info.clone(), "phone".as_bytes().to_vec());
            ensure!(phone.len() >= 10, Error::<T>::FundPhoneTooShort);
            ensure!(phone.len() <= 21, Error::<T>::FundPhoneTooLong);
            ensure!(validate_phonenumber(phone), Error::<T>::FundPhoneIsWrong);
            let ipfsdocs = json_get_complexarray(info.clone(), "ipfsdocs".as_bytes().to_vec());
            if ipfsdocs.len() > 2 {
                let mut x = 0;
                loop {
                    let w = json_get_recordvalue(ipfsdocs.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    let description = json_get_value(w.clone(), "description".as_bytes().to_vec());
                    ensure!(
                        description.len() > 5,
                        Error::<T>::FundDocumentDescriptionTooShort
                    );
                    let ipfsaddress = json_get_value(w.clone(), "ipfsaddress".as_bytes().to_vec());
                    ensure!(
                        ipfsaddress.len() > 20,
                        Error::<T>::FundDocumentIpfsAddressTooShort
                    );

                    x += 1;
                }
                ensure!(x > 0, Error::<T>::FundMissingDocuments);
            }
            // check for Initial fees (considering 2 decimals as integer)
            let initialfees = json_get_value(info.clone(), "initialfees".as_bytes().to_vec());
            let initialfeesv = vecu8_to_u32(initialfees);
            ensure!(initialfeesv > 0, Error::<T>::FundInitialFeesCannotBeZero);
            // check for yearly fees on capital (considering 2 decimals as integer)
            let yearlyfees = json_get_value(info.clone(), "yearlyfees".as_bytes().to_vec());
            let yearlyfeesv = vecu8_to_u32(yearlyfees);
            ensure!(yearlyfeesv > 0, Error::<T>::FundYearlyFeesCannotBeZero);
            // check for performance fees on interest (considering 2 decimals as integer)
            let performancefees =
                json_get_value(info.clone(), "performancefees".as_bytes().to_vec());
            let performancefeesv = vecu8_to_u32(performancefees);
            ensure!(
                performancefeesv > 0,
                Error::<T>::FundPerformanceFeesCannotBeZero
            );
            // check fund type E==Enterprise Fund  H==Hedge Fund
            let fundtype = json_get_value(info.clone(), "fundtype".as_bytes().to_vec());
            ensure!(
                fundtype[0] == b'E' || fundtype[0] == b'H',
                Error::<T>::FundTypeIsWrong
            );
            // check deposit account id for the fund (Wallet Address)
            let depositaccountid =
                json_get_value(info.clone(), "depositaccountid".as_bytes().to_vec());
            ensure!(
                depositaccountid.len() == 48,
                Error::<T>::FundDepositAccountIdIsWrong
            );
            // check fund managers id for the fund (Wallet Addresses) - At the least one is mandatory
            let fundmanagers =
                json_get_complexarray(info.clone(), "fundmanagers".as_bytes().to_vec());
            let mut x = 0;
            if fundmanagers.len() > 2 {
                loop {
                    let w = json_get_recordvalue(fundmanagers.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    ensure!(w.len() == 48, Error::<T>::FundManagerAccountIdIsWrong);
                    x += 1;
                }
            }
            ensure!(x > 0, Error::<T>::FundManagerAccountisMissing);
            //store Fund on chain
            if Funds::<T>::contains_key(&accountid) {
                // check that is not already approved from anybody
                let itr = FundsSignatures::<T>::iter_prefix(accountid.clone());
                ensure!(
                    itr.count() == 0,
                    Error::<T>::FundUnderProcessItCannotBeChanged
                );
                // Replace Fund Data
                Funds::<T>::take(accountid.clone());
            }
            Funds::<T>::insert(accountid.clone(), info.clone());
            // Generate event
            Self::deposit_event(Event::FundStored(accountid, info));
            // Return a successful DispatchResult
            Ok(())
        }
        // This function allow the manager/supervisors for fund approval, to approve a new fund
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn fund_approve(origin: OriginFor<T>, accountid: T::AccountId) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            let mut signingtype = 0;
            //check that the fund Id exists
            ensure!(
                Funds::<T>::contains_key(&accountid),
                Error::<T>::KycIdNotFound
            );
            // check for possibile duplicated signatures
            ensure!(
                !FundsSignatures::<T>::contains_key(&accountid, &signer),
                Error::<T>::FundsSignatureAlreadyPresentrSameSigner
            );
            // check the signer is one of the operators for fund approval
            let json: Vec<u8> = Settings::<T>::get("fundapproval".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                    signingtype = 1;
                }
            }
            let supervisor = json_get_value(json.clone(), "supervisor".as_bytes().to_vec());
            if !supervisor.is_empty() {
                let supervisorvec = bs58::decode(supervisor).into_vec().unwrap();
                let accountidsupervisor = T::AccountId::decode(&mut &supervisorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidsupervisor {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 2;
                    }
                }
            }
            ensure!(flag == 1, Error::<T>::SignerIsNotAuthorizedForFundApproval);
            // write/update signature
            FundsSignatures::<T>::insert(accountid.clone(), signer.clone(), signingtype);
            // check for all the approval
            let mut sigmanager = 0;
            let mut sigsupervisor = 0;
            let mut itr = FundsSignatures::<T>::iter_prefix(accountid.clone());
            let mut result;
            loop {
                result = itr.next();
                match result {
                    Some(x) => {
                        if x.1 == 1 {
                            sigmanager = 1;
                        }
                        if x.1 == 2 {
                            sigsupervisor = 1;
                        }
                    }
                    None => break,
                }
            }
            // store approved flag if all signatures have been received
            if sigmanager == 1 && sigsupervisor == 1 {
                FundsApproved::<T>::insert(accountid.clone(), 1);
                // generate event for approved
                Self::deposit_event(Event::FundApproved(accountid.clone(), signer.clone()));
            }
            // generate event for the approval
            Self::deposit_event(Event::FundSignedforApproval(accountid, signer));
            // Return a successful DispatchResult
            Ok(())
        }
        // Function to create a new bond subject to approval. The info field is a json structure with the following fields:
        // totalamount: total amount considering 0 decimals
        // currency: is the currency code as from the blockchain storage "Currencies"
        // country: is the the iso conty code as from blockchain storage "IsoCountries"
        // interestrate: is the interest rate expressed in an integer assumin 2 decimals, for example 200 is equivalent to 2.00 %
        // interest type: X=Fixed Rate / F=Floating Rate /Z= Zero Interest/ I= Inflation Linked
        // for example:
        //
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn bond_create_change(origin: OriginFor<T>, id: u32, info: Vec<u8>) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            //check id >0
            ensure!(id > 0, Error::<T>::BondIdIsWrongCannotBeZero);
            // check that the signer is the Owner of an authorized Fund
            ensure!(
                Funds::<T>::contains_key(&signer),
                Error::<T>::FundNotFoundforSigner
            );
            ensure!(
                FundsApproved::<T>::contains_key(&signer),
                Error::<T>::FundNotYetApproved
            );
            // check the signer has been subject has a KYC
            ensure!(
                Kyc::<T>::contains_key(&signer),
                Error::<T>::MissingKycForSigner
            );
            // check the Kyc has been approved
            ensure!(
                KycApproved::<T>::contains_key(&signer),
                Error::<T>::KycSignerIsNotApproved
            );
            // check owner field
            let owner = json_get_value(info.clone(), "owner".as_bytes().to_vec());
            ensure!(owner.len() == 48, Error::<T>::OwnerAccountIsMissing);
            let ownervec = bs58::decode(owner).into_vec().unwrap();
            let accountidowner =
                T::AccountId::decode(&mut &ownervec[1..33]).map_err(|_| Error::<T>::InvalidJson)?;
            // check that the owner matches the signer of the transaction
            ensure!(
                signer == accountidowner,
                Error::<T>::SignerIsNotMatchingOwnerAccount
            );
            // check if the bond exists is not yet under approval and belong to the signer
            if Bonds::<T>::contains_key(&id) {
                // check there are not yet approval signatures
                let itrs = BondsSignatures::<T>::iter_prefix(id);
                ensure!(
                    itrs.count() == 0,
                    Error::<T>::BondUnderApprovalCannotBeChanged
                );
                // check owner stored is matching the signer
                let cinfo = Bonds::<T>::get(&id).unwrap();
                let cowner = json_get_value(cinfo.clone(), "owner".as_bytes().to_vec());
                let cownervec = bs58::decode(cowner).into_vec().unwrap();
                let caccountidowner = T::AccountId::decode(&mut &cownervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                ensure!(
                    caccountidowner == signer,
                    Error::<T>::SignerIsNotMatchingOwnerAccount
                );
            }
            // check total amount >0
            let totalamount = json_get_value(info.clone(), "totalamount".as_bytes().to_vec());
            let totalamountv = vecu8_to_u32(totalamount);
            ensure!(totalamountv > 0, Error::<T>::BondTotalAmountCannotBeZero);
            // check currency is one of the ISO set
            let currency = json_get_value(info.clone(), "currency".as_bytes().to_vec());
            ensure!(
                Currencies::<T>::contains_key(&currency),
                Error::<T>::CurrencyCodeNotFound
            );
            // check country is a valid ISO set
            let country = json_get_value(info.clone(), "country".as_bytes().to_vec());
            ensure!(
                IsoCountries::<T>::contains_key(&country),
                Error::<T>::CountryCodeNotFound
            );
            let country = json_get_value(info.clone(), "country".as_bytes().to_vec());
            ensure!(
                IsoCountries::<T>::contains_key(&country),
                Error::<T>::CountryCodeNotFound
            );
            // check interest rate >0 considering 2 decimals as integer
            let interestrate = json_get_value(info.clone(), "interestrate".as_bytes().to_vec());
            let interestratev = vecu8_to_u32(interestrate);
            let interestype = json_get_value(info.clone(), "interestype".as_bytes().to_vec());
            ensure!(
                interestratev > 0 || (interestratev == 0 && interestype[0] == b'Z'),
                Error::<T>::BondInterestRateIsWrong
            ); // Zero Interest Rate must be possible
               // check interest type where X= FiXed Rate, F=Floating Rate, Z= Zero Interest Rate, I=Inflation Linked Rate
            ensure!(
                interestype[0] == b'X'
                    || interestype[0] == b'F'
                    || interestype[0] == b'Z'
                    || interestype[0] == b'I',
                Error::<T>::BondInterestTypeIsWrong
            );
            // check maturity in months from the approval time
            let maturity = json_get_value(info.clone(), "maturity".as_bytes().to_vec());
            let maturityv = vecu8_to_u32(maturity);
            ensure!(maturityv > 0, Error::<T>::BondMaturityCannotBeZero);
            // 50 years maximum for any bond type
            ensure!(maturityv <= 600, Error::<T>::BondMaturityTooLong);
            // check Instalments
            let instalments = json_get_value(info.clone(), "instalments".as_bytes().to_vec());
            let instalmentsv = vecu8_to_u32(instalments);
            ensure!(instalmentsv <= 600, Error::<T>::BondTooManyInstalments);
            ensure!(
                instalmentsv <= maturityv,
                Error::<T>::BondInstalmentsCannotExceedMaturity
            );
            // check Grace Period
            let graceperiod = json_get_value(info.clone(), "graceperiod".as_bytes().to_vec());
            let graceperiodv = vecu8_to_u32(graceperiod);
            ensure!(
                graceperiodv < maturityv,
                Error::<T>::BondGracePeriodCannotExceedMaturity
            );
            // check accepted currencies
            let acceptedcurrencies =
                json_get_value(info.clone(), "acceptedcurrencies".as_bytes().to_vec());
            if acceptedcurrencies.len() > 2 {
                let mut x = 0;
                loop {
                    let ac = json_get_arrayvalue(acceptedcurrencies.clone(), x);
                    if ac.is_empty() {
                        break;
                    }
                    // check crypto currency on blockchain
                    ensure!(
                        Currencies::<T>::contains_key(&ac),
                        Error::<T>::CurrencyCodeNotFound
                    );
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::BondAcceptedCurrenciesCannotBeEmpty);
            }
            // check subordinated field Y/N
            let subordinated = json_get_value(info.clone(), "subordinated".as_bytes().to_vec());
            ensure!(
                subordinated[0] == b'Y' || subordinated[0] == b'N',
                Error::<T>::BondSubordinatedIsWrong
            );
            // check put option field Y/N
            let putoption = json_get_value(info.clone(), "putoption".as_bytes().to_vec());
            ensure!(
                putoption[0] == b'Y' || putoption[0] == b'N',
                Error::<T>::BondPutOptionIsWrong
            );
            // check vesting period for put option
            if putoption[0] == b'Y' {
                let putvestingperiod =
                    json_get_value(info.clone(), "putvestingperiod".as_bytes().to_vec());
                let putvestingperiodv = vecu8_to_u32(putvestingperiod);
                ensure!(
                    putvestingperiodv > 0,
                    Error::<T>::BondPutVestingPeriodCannotBeZero
                );
            }
            // check call option field Y/N
            let calloption = json_get_value(info.clone(), "calloption".as_bytes().to_vec());
            ensure!(
                calloption[0] == b'Y' || calloption[0] == b'N',
                Error::<T>::BondCallOptionIsWrong
            );
            // check vesting period for call option
            if calloption[0] == b'Y' {
                let callvestingperiod =
                    json_get_value(info.clone(), "callvestingperiod".as_bytes().to_vec());
                let callvestingperiodv = vecu8_to_u32(callvestingperiod);
                ensure!(
                    callvestingperiodv > 0,
                    Error::<T>::BondCallVestingPeriodCannotBeZero
                );
            }
            // check put convertible option field Y/N
            let putconvertibleoption =
                json_get_value(info.clone(), "putconvertibleoption".as_bytes().to_vec());
            ensure!(
                putconvertibleoption[0] == b'Y' || putconvertibleoption[0] == b'N',
                Error::<T>::BondPutConvertibleOptionIsWrong
            );
            // check call convertible option field Y/N
            let callconvertibleoption =
                json_get_value(info.clone(), "callconvertibleoption".as_bytes().to_vec());
            ensure!(
                callconvertibleoption[0] == b'Y' || callconvertibleoption[0] == b'N',
                Error::<T>::BondCallConvertibleOptionIsWrong
            );
            // check the info documents
            // get required documents as from settings
            let mut settingdocs = "".as_bytes().to_vec();
            let mut settingconf = 0;
            let mut ndocuments = 0;
            if Settings::<T>::contains_key("infodocuments".as_bytes().to_vec()) {
                settingdocs = Settings::<T>::get("infodocuments".as_bytes().to_vec()).unwrap();
                settingconf = 1;
                let documents =
                    json_get_complexarray(settingdocs.clone(), "documents".as_bytes().to_vec());
                if documents.len() > 2 {
                    loop {
                        let w = json_get_recordvalue(documents.clone(), ndocuments);
                        if w.is_empty() {
                            break;
                        }
                        ndocuments += 1;
                    }
                }
            }
            let ipfsdocs = json_get_complexarray(info.clone(), "ipfsdocs".as_bytes().to_vec());
            if ipfsdocs.len() > 2 {
                let mut x = 0;
                loop {
                    let w = json_get_recordvalue(ipfsdocs.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    let description = json_get_value(w.clone(), "description".as_bytes().to_vec());
                    ensure!(
                        description.len() > 5,
                        Error::<T>::BondDocumentDescriptionTooShort
                    );
                    let ipfsaddress = json_get_value(w.clone(), "ipfsaddress".as_bytes().to_vec());
                    ensure!(
                        ipfsaddress.len() > 20,
                        Error::<T>::BondDocumentIpfsAddressTooShort
                    );
                    //check if one of the mandatory documents
                    if settingconf == 1 {
                        let documents = json_get_complexarray(
                            settingdocs.clone(),
                            "documents".as_bytes().to_vec(),
                        );
                        if documents.len() > 2 {
                            loop {
                                let ww = json_get_recordvalue(documents.clone(), ndocuments);
                                if ww.is_empty() {
                                    break;
                                }
                                let wdescription =
                                    json_get_value(ww.clone(), "description".as_bytes().to_vec());
                                if wdescription == description {
                                    ndocuments -= 1;
                                }
                            }
                        }
                    }
                    x += 1;
                }
                ensure!(x > 0 && ndocuments == 0, Error::<T>::BondMissingDocuments);
            }
            // remove previous data if any
            if Bonds::<T>::contains_key(&id) {
                Bonds::<T>::take(&id);
            }
            //store bond
            Bonds::<T>::insert(id, info.clone());
            // Generate event
            Self::deposit_event(Event::BondCreated(signer, id, info));
            // Return a successful DispatchResult
            Ok(())
        }
        // function to approve the bond from manager or members of the committee
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn bond_approve(origin: OriginFor<T>, bondid: u32) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            let mut signingtype = 0;
            //check id >0
            ensure!(
                Bonds::<T>::contains_key(&bondid),
                Error::<T>::BondsIdNotFound
            );
            // check for already approved bond
            ensure!(
                !BondsApproved::<T>::contains_key(bondid),
                Error::<T>::BondAlreadyApproved
            );
            //check for duplicated signatures
            ensure!(
                !BondsSignatures::<T>::contains_key(&bondid, &signer),
                Error::<T>::BondsSignatureAlreadyPresentrSameSigner
            );
            // check the signer is one of the operators for Bonds approval
            let json: Vec<u8> = Settings::<T>::get("bondapproval".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                    signingtype = 1;
                }
            }
            let committee = json_get_complexarray(json, "committee".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let committeem = json_get_arrayvalue(committee.clone(), x);
                if committeem.is_empty() {
                    break;
                }
                let committeemvec = bs58::decode(committeem).into_vec().unwrap();
                let accountidoperator = T::AccountId::decode(&mut &committeemvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if accountidoperator == signer {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 2;
                    }
                }
                x += 1;
            }
            ensure!(flag == 1, Error::<T>::SignerIsNotAuthorizedForBondApproval);
            // write/update signature
            BondsSignatures::<T>::insert(bondid, signer.clone(), signingtype);
            // check for all the approval
            // TODO? actually one committe member is enough to reach the "approved" status. It may be necessary to let sign a minimum quorum
            let mut sigmanager = 0;
            let mut sigcommitee = 0;
            let mut itr = BondsSignatures::<T>::iter_prefix(bondid);
            let mut result;
            loop {
                result = itr.next();
                match result {
                    Some(x) => {
                        if x.1 == 1 {
                            sigmanager = 1;
                        }
                        if x.1 == 2 {
                            sigcommitee = 1;
                        }
                    }
                    None => break,
                }
            }
            // store approved flag if all signatures have been received
            if sigmanager == 1 && sigcommitee == 1 {
                BondsApproved::<T>::insert(bondid, 1);
                // generate event for approved
                Self::deposit_event(Event::BondApproved(bondid, signer.clone()));
            }
            // generate event for the approval
            Self::deposit_event(Event::BondSignedforApproval(bondid, signer));
            // Return a successful DispatchResult
            Ok(())
        }
        // function to a bond paying for the amount requested
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn bond_subscribe(origin: OriginFor<T>, bondid: u32, amount: u32) -> DispatchResult {
            // veryfy the transaction is signed
            let signer = ensure_signed(origin)?;
            //check bondid
            ensure!(
                Bonds::<T>::contains_key(&bondid),
                Error::<T>::BondsIdNotFound
            );
            // check for already approved bond
            ensure!(
                !BondsApproved::<T>::contains_key(bondid),
                Error::<T>::BondAlreadyApproved
            );
            // get the currency address (tokenid) from the bond
            let bondv = Bonds::<T>::get(&bondid).unwrap();
            let currency = json_get_value(bondv.clone(), "currency".as_bytes().to_vec());
            // search for the currency
            ensure!(
                !Currencies::<T>::contains_key(currency.clone()),
                Error::<T>::CurrencyCodeNotFound
            );
            let currencyv = Currencies::<T>::get(currency).unwrap();
            let tokenidv = json_get_value(currencyv, "address".as_bytes().to_vec());
            let tokenid = vecu8_to_u32(tokenidv);
            ensure!(tokenid > 0, Error::<T>::CurrencyCodeHasNotValidAddress);
            // check for enough balance in the stable coin
            let depositstablecoin = pallet_assets::Pallet::<T>::balance(tokenid, signer.clone());
            ensure!(
                depositstablecoin >= amount.into(),
                Error::<T>::InsufficientFunds
            );
            // check for quantity requested + total subscribe <= total bond amount
            let totalbondamount = json_get_value(bondv.clone(), "totalamount".as_bytes().to_vec());
            let totalbondamountv = vecu8_to_u32(totalbondamount);
            let mut totalshares = BondsTotalShares::<T>::get(bondid).unwrap();
            ensure!(
                totalbondamountv >= totalshares + amount,
                Error::<T>::TotalShareAvailablesNotEnough
            );
            // get bond fund account
            let owner = json_get_value(bondv.clone(), "owner".as_bytes().to_vec());
            ensure!(owner.len() == 48, Error::<T>::OwnerAccountIsMissing);
            let ownervec = bs58::decode(owner).into_vec().unwrap();
            let accountidowner =
                T::AccountId::decode(&mut &ownervec[1..33]).map_err(|_| Error::<T>::InvalidJson)?;
            // transfer amount of stable coins to the Bond Owner account
            pallet_assets::Pallet::<T>::transfer(
                RawOrigin::Signed(signer.clone()).into(),
                bondid,
                T::Lookup::unlookup(accountidowner.clone()),
                amount.into(),
            )
            .unwrap();
            // mint the shares
            let mut sharessubscribed = 0;
            if BondsShares::<T>::contains_key(bondid, signer.clone()) {
                sharessubscribed = BondsShares::<T>::get(bondid, signer.clone()).unwrap();
                let tss = sharessubscribed + amount;
                BondsShares::<T>::take(bondid, signer.clone());
                BondsShares::<T>::insert(bondid, signer.clone(), tss);
            } else {
                BondsShares::<T>::insert(bondid, signer.clone(), amount + sharessubscribed);
            }
            // update the total shares subscribed for the bond.
            if totalshares > 0 {
                BondsTotalShares::<T>::take(bondid);
            }
            totalshares = totalshares + amount;
            BondsTotalShares::<T>::insert(bondid, totalshares);

            // generate event for the subscription
            Self::deposit_event(Event::BondSubscribed(bondid, signer, amount));
            // Return a successful DispatchResult
            Ok(())
        }
        // this function has the purpose to the insert or update data for Credit Rating Agencies
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn credit_rating_agency_create_change(
            origin: OriginFor<T>,
            accountid: T::AccountId,
            info: Vec<u8>,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // check the signer is one of the manager or a member of the committee
            // read configuration on chain
            ensure!(
                Settings::<T>::contains_key("creditratingagencies".as_bytes().to_vec()),
                Error::<T>::SettingsDoesNotExist
            );
            let json: Vec<u8> =
                Settings::<T>::get("creditratingagencies".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            let mut signingtype = 0;
            // check for manager
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                    signingtype = 1;
                }
            }
            // check for member of the commitee
            let operators = json_get_complexarray(json, "committee".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let operator = json_get_arrayvalue(operators.clone(), x);
                if operator.is_empty() {
                    break;
                }
                let operatorvec = bs58::decode(operator).into_vec().unwrap();
                let accountidoperator = T::AccountId::decode(&mut &operatorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if accountidoperator == signer {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 3;
                    }
                }
                x += 1;
            }
            ensure!(
                flag == 1,
                Error::<T>::SignerIsNotAuthorizedForCreditRatingAgencySubmission
            );
            //check info length
            ensure!(
                info.len() < 8192,
                Error::<T>::CreditRatingAgencyInfoIsTooLong
            );
            // check json validity
            let js = info.clone();
            ensure!(json_check_validity(js), Error::<T>::InvalidJson);
            // check name
            let name = json_get_value(info.clone(), "name".as_bytes().to_vec());
            ensure!(name.len() >= 10, Error::<T>::CreditRatingAgencyNameTooShort);
            ensure!(name.len() <= 64, Error::<T>::CreditRatingAgencyNameTooLong);
            // check Website
            let website = json_get_value(info.clone(), "website".as_bytes().to_vec());
            ensure!(
                validate_weburl(website),
                Error::<T>::CreditRatingAgencyWebSiteIsWrong
            );
            let ipfsdocs = json_get_complexarray(info.clone(), "ipfsdocs".as_bytes().to_vec());
            if ipfsdocs.len() > 2 {
                let mut x = 0;
                loop {
                    let w = json_get_recordvalue(ipfsdocs.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    let description = json_get_value(w.clone(), "description".as_bytes().to_vec());
                    ensure!(
                        description.len() > 5,
                        Error::<T>::CreditRatingAgencyDocumentDescriptionTooShort
                    );
                    let ipfsaddress = json_get_value(w.clone(), "ipfsaddress".as_bytes().to_vec());
                    ensure!(
                        ipfsaddress.len() > 20,
                        Error::<T>::CreditRatingAgencyDocumentIpfsAddressTooShort
                    );
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::CreditRatingAgencyMissingDocuments);
            }
            if CreditRatingAgencies::<T>::contains_key(&accountid) {
                // Replace Credit Rating Agency Data
                CreditRatingAgencies::<T>::take(accountid.clone());
            }
            CreditRatingAgencies::<T>::insert(accountid.clone(), info.clone());
            // Generate event
            Self::deposit_event(Event::CreditRatingAgencyStored(accountid, info));
            // Return a successful DispatchResult
            Ok(())
        }
        // this function has the purpose to remove a Rating Agency from the state. Only the manager is enabled
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn credit_rating_agency_destroy(
            origin: OriginFor<T>,
            accountid: T::AccountId,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // check that the credit agency exists.
            ensure!(
                CreditRatingAgencies::<T>::contains_key(&accountid),
                Error::<T>::CreditRatingAgencyNotFound
            );
            // check the signer is one of the manager
            // read configuration on chain
            ensure!(
                Settings::<T>::contains_key("creditratingagencies".as_bytes().to_vec()),
                Error::<T>::SettingsDoesNotExist
            );
            let json: Vec<u8> =
                Settings::<T>::get("creditratingagencies".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            // check for manager
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                }
            }
            ensure!(
                flag == 1,
                Error::<T>::SignerIsNotAuthorizedForCreditRatingAgencyCancellation
            );
            // cancel the credit agency
            CreditRatingAgencies::<T>::take(accountid.clone());
            // Generate event
            Self::deposit_event(Event::CreditRatingAgencyDestroyed(accountid));
            // Return a successful DispatchResult
            Ok(())
        }
        // this function has the purpose to the insert or update data for Credit Rating
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn credit_rating_create(
            origin: OriginFor<T>,
            bondid: u32,
            info: Vec<u8>,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // check the signer is a credit rating agency
            ensure!(
                CreditRatingAgencies::<T>::contains_key(&signer),
                Error::<T>::SignerIsNotAuthorizedAsCreditRatingAgency
            );
            //check info length
            ensure!(info.len() < 8192, Error::<T>::CreditRatingInfoIsTooLong);
            // check json validity
            let js = info.clone();
            ensure!(json_check_validity(js), Error::<T>::InvalidJson);
            // check description of the rating
            let description = json_get_value(info.clone(), "description".as_bytes().to_vec());
            ensure!(
                description.len() >= 10,
                Error::<T>::CreditRatingDescriptionTooShort
            );
            ensure!(
                description.len() <= 64,
                Error::<T>::CreditRatingDescriptionTooLong
            );
            let ipfsdocs = json_get_complexarray(info.clone(), "ipfsdocs".as_bytes().to_vec());
            // check for the presence of Documents
            ensure!(
                ipfsdocs.len() > 2,
                Error::<T>::CreditRatingDocumentsAreMissing
            );
            if ipfsdocs.len() > 2 {
                let mut x = 0;
                loop {
                    let w = json_get_recordvalue(ipfsdocs.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    let description = json_get_value(w.clone(), "description".as_bytes().to_vec());
                    ensure!(
                        description.len() > 5,
                        Error::<T>::CreditRatingDocumentDescriptionTooShort
                    );
                    let ipfsaddress = json_get_value(w.clone(), "ipfsaddress".as_bytes().to_vec());
                    ensure!(
                        ipfsaddress.len() > 20,
                        Error::<T>::CreditRatingDocumentIpfsAddressTooShort
                    );
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::CreditRatingMissingDocuments);
            }
            // Insert Credit Rating
            CreditRatings::<T>::insert(bondid, info.clone());
            // Generate event
            Self::deposit_event(Event::CreditRatingStored(bondid, info));
            // Return a successful DispatchResult
            Ok(())
        }
        // this function has the purpose to the insert collaterals document for a bond
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn collaterals_create(
            origin: OriginFor<T>,
            bondid: u32,
            collateralid: u32,
            info: Vec<u8>,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // check for bond id
            ensure!(
                Bonds::<T>::contains_key(&bondid),
                Error::<T>::BondsIdNotFound
            );
            // check for collateral id  not already present
            ensure!(
                !Collaterals::<T>::contains_key(&bondid, &collateralid),
                Error::<T>::CollateralIdAlreadyPresent
            );
            // check the signer is the owner of the bond
            if Bonds::<T>::contains_key(&bondid) {
                // check owner stored is matching the signer
                let cinfo = Bonds::<T>::get(&bondid).unwrap();
                let cowner = json_get_value(cinfo.clone(), "owner".as_bytes().to_vec());
                let cownervec = bs58::decode(cowner).into_vec().unwrap();
                let caccountidowner = T::AccountId::decode(&mut &cownervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                ensure!(
                    caccountidowner == signer,
                    Error::<T>::SignerIsNotMatchingOwnerAccount
                );
            }
            //check info length
            ensure!(info.len() < 8192, Error::<T>::CollateralsInfoIsTooLong);
            // check json validity
            let js = info.clone();
            ensure!(json_check_validity(js), Error::<T>::InvalidJson);
            // check documents
            let ipfsdocs = json_get_complexarray(info.clone(), "ipfsdocs".as_bytes().to_vec());
            if ipfsdocs.len() > 2 {
                let mut x = 0;
                loop {
                    let w = json_get_recordvalue(ipfsdocs.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    let description = json_get_value(w.clone(), "description".as_bytes().to_vec());
                    ensure!(
                        description.len() > 5,
                        Error::<T>::CollateralDocumentDescriptionTooShort
                    );
                    let ipfsaddress = json_get_value(w.clone(), "ipfsaddress".as_bytes().to_vec());
                    ensure!(
                        ipfsaddress.len() > 20,
                        Error::<T>::CollateralDocumentIpfsAddressTooShort
                    );
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::CollateralMissingDocuments);
            }
            // Insert Collateral
            Collaterals::<T>::insert(bondid, collateralid, info.clone());
            // Generate event
            Self::deposit_event(Event::CollateralsStored(bondid, collateralid, info));
            // Return a successful DispatchResult
            Ok(())
        }
        // this function has the purpose to remove the collateral from the owner of the bond
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn collaterals_destroy(
            origin: OriginFor<T>,
            bondid: u32,
            collateralid: u32,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // check for bond id
            ensure!(
                Bonds::<T>::contains_key(&bondid),
                Error::<T>::BondsIdNotFound
            );
            // check the signer is the owner of the bond
            if Bonds::<T>::contains_key(&bondid) {
                // check owner stored is matching the signer
                let cinfo = Bonds::<T>::get(&bondid).unwrap();
                let cowner = json_get_value(cinfo.clone(), "owner".as_bytes().to_vec());
                let cownervec = bs58::decode(cowner).into_vec().unwrap();
                let caccountidowner = T::AccountId::decode(&mut &cownervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                ensure!(
                    caccountidowner == signer,
                    Error::<T>::SignerIsNotMatchingOwnerAccount
                );
            }
            // check for collateral id is already present
            ensure!(
                Collaterals::<T>::contains_key(&bondid, &collateralid),
                Error::<T>::CollateralIdNotFound
            );
            // check the collateral has not yet approved
            ensure!(
                !CollateralsApproval::<T>::contains_key(bondid, collateralid),
                Error::<T>::CollateralAlreadyApproved
            );
            // Delete Collateral
            Collaterals::<T>::take(bondid, collateralid);
            // Generate event
            Self::deposit_event(Event::CollateralsDestroyed(bondid, collateralid));
            // Return a successful DispatchResult
            Ok(())
        }
        // this function has the purpose to the insert collaterals document for a bond
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn collaterals_approve(
            origin: OriginFor<T>,
            bondid: u32,
            collateralid: u32,
            info: Vec<u8>,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // check the signer is one of the manager or a member of the committee
            ensure!(
                Settings::<T>::contains_key("collateralsverification".as_bytes().to_vec()),
                Error::<T>::SettingsDoesNotExist
            );
            let json: Vec<u8> =
                Settings::<T>::get("collateralsverification".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            let mut signingtype = 0;
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                    signingtype = 1;
                }
            }
            let operators = json_get_complexarray(json, "committee".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let operator = json_get_arrayvalue(operators.clone(), x);
                if operator.is_empty() {
                    break;
                }
                let operatorvec = bs58::decode(operator).into_vec().unwrap();
                let accountidoperator = T::AccountId::decode(&mut &operatorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if accountidoperator == signer {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 3;
                    }
                }
                x += 1;
            }
            ensure!(
                flag == 1,
                Error::<T>::SignerIsNotAuthorizedForCollateralsApproval
            );
            // check for bond id
            ensure!(
                Bonds::<T>::contains_key(&bondid),
                Error::<T>::BondsIdNotFound
            );
            // check for collateral id  not already present
            ensure!(
                !CollateralsApproval::<T>::contains_key(&bondid, &collateralid),
                Error::<T>::CollateralIdAlreadyPresent
            );
            //check info length
            ensure!(info.len() < 8192, Error::<T>::CollateralsInfoIsTooLong);
            // check json validity
            let js = info.clone();
            ensure!(json_check_validity(js), Error::<T>::InvalidJson);
            // check documents
            let ipfsdocs = json_get_complexarray(info.clone(), "ipfsdocs".as_bytes().to_vec());
            if ipfsdocs.len() > 2 {
                let mut x = 0;
                loop {
                    let w = json_get_recordvalue(ipfsdocs.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    let description = json_get_value(w.clone(), "description".as_bytes().to_vec());
                    ensure!(
                        description.len() > 5,
                        Error::<T>::CollateralDocumentDescriptionTooShort
                    );
                    let ipfsaddress = json_get_value(w.clone(), "ipfsaddress".as_bytes().to_vec());
                    ensure!(
                        ipfsaddress.len() > 20,
                        Error::<T>::CollateralDocumentIpfsAddressTooShort
                    );
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::CollateralMissingDocuments);
            }
            // Insert Collateral
            CollateralsApproval::<T>::insert(bondid, collateralid, info.clone());
            // Generate event
            Self::deposit_event(Event::CollateralsApproved(bondid, collateralid, info));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Create a new Iso country code and name
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn iso_country_create(
            origin: OriginFor<T>,
            countrycode: Vec<u8>,
            countryname: Vec<u8>,
        ) -> DispatchResult {
            // check the request is signed from the Super User
            let _sender = ensure_root(origin)?;
            // check country code length == 2
            ensure!(countrycode.len() == 2, Error::<T>::WrongLengthCountryCode);
            // check country name length  >= 3
            ensure!(countryname.len() >= 3, Error::<T>::CountryNameTooShort);
            // check the country is not alreay present on chain
            ensure!(
                !IsoCountries::<T>::contains_key(&countrycode),
                Error::<T>::CountryCodeAlreadyPresent
            );
            // store the Iso Country Code and Name
            IsoCountries::<T>::insert(countrycode.clone(), countryname.clone());
            // Generate event
            Self::deposit_event(Event::IsoCountryCreated(countrycode, countryname));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Destroy an Iso country code and name
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn iso_country_destroy(origin: OriginFor<T>, countrycode: Vec<u8>) -> DispatchResult {
            // check the request is signed from the Super User
            let _sender = ensure_root(origin)?;
            // verify the country code exists
            ensure!(
                IsoCountries::<T>::contains_key(&countrycode),
                Error::<T>::CountryCodeNotFound
            );
            // Remove country code
            IsoCountries::<T>::take(countrycode.clone());
            // Generate event
            //it can leave orphans, anyway it's a decision of the super user
            Self::deposit_event(Event::IsoCountryDestroyed(countrycode));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Create a new Currency code with name and other info in a json structure
        /// {"name":"Bitcoin","category":"c(rypto)/f(iat)","country":"countryisocode","blockchain":"Ethereum(...)","address":"xxxfor_crypto_currencyxxx"}
        /// for example: {"name":"Bitcoin","category":"c","country":"AE","blockchain":"Bitcoin","address":"not applicable"}
        /// {"name":"American Dollars","category":"f","country":"US","blockchain":"not applicable","address":"not applicable"}
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn currency_create(
            origin: OriginFor<T>,
            currencycode: Vec<u8>,
            info: Vec<u8>,
        ) -> DispatchResult {
            // check the request is signed from the Super User
            let _sender = ensure_root(origin)?;
            // check currency code length is between 3 and 5 bytes
            ensure!(
                (currencycode.len() == 2),
                Error::<T>::WrongLengthCurrencyCode
            );
            // check the info field is not longer 1024 bytes
            ensure!((info.len() <= 1024), Error::<T>::SizeInfoTooLong);
            // check for a valid json structure
            ensure!(json_check_validity(info.clone()), Error::<T>::InvalidJson);
            // check for name
            let name = json_get_value(info.clone(), "name".as_bytes().to_vec());
            ensure!(name.len() >= 3, Error::<T>::CurrencyNameTooShort);
            ensure!(name.len() <= 32, Error::<T>::CurrencyNameTooLong);
            // check for type of currency (fiat/crypto)
            let category = json_get_value(info.clone(), "category".as_bytes().to_vec());
            let c: Vec<u8> = vec![b'c'];
            let f: Vec<u8> = vec![b'f'];
            ensure!(
                (category == c || category == f),
                Error::<T>::CurrencyCategoryIswrong
            );
            // check for the country code in case of Fiat currency
            if category == f {
                let countrycode = json_get_value(info.clone(), "country".as_bytes().to_vec());
                ensure!(
                    IsoCountries::<T>::contains_key(&countrycode),
                    Error::<T>::CountryCodeNotFound
                );
            }
            // check for the blockchain in case of Crypto currency
            if category == c {
                let blockchain = json_get_value(info.clone(), "blockchain".as_bytes().to_vec());
                ensure!(blockchain.len() >= 3, Error::<T>::BlockchainNameTooShort);
                ensure!(blockchain.len() <= 32, Error::<T>::BlockchainNameTooLong);
            }
            // check the currency is not alreay present on chain
            ensure!(
                !Currencies::<T>::contains_key(&currencycode),
                Error::<T>::CurrencyCodeAlreadyPresent
            );
            // store the Currency Code and info
            Currencies::<T>::insert(currencycode.clone(), info.clone());
            // Generate event
            Self::deposit_event(Event::CurrencyCodeCreated(currencycode, info));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Destroy a currency
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn currency_destroy(origin: OriginFor<T>, currencycode: Vec<u8>) -> DispatchResult {
            // check the request is signed from the Super User
            let _sender = ensure_root(origin)?;
            // verify the currency code exists
            ensure!(
                Currencies::<T>::contains_key(&currencycode),
                Error::<T>::CurrencyCodeNotFound
            );
            // Remove currency code
            Currencies::<T>::take(currencycode.clone());
            // Generate event
            //it can leave orphans, anyway it's a decision of the super user
            Self::deposit_event(Event::CurrencyDestroyed(currencycode));
            // Return a successful DispatchResult
            Ok(())
        }

        /// Create an Underwriter
        /// Suggestion: An approval process with multiple signatures may be considered useful.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn undwerwriter_create(
            origin: OriginFor<T>,
            underwriter_account: T::AccountId,
            info: Vec<u8>,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // check for a valid json structure
            ensure!(json_check_validity(info.clone()), Error::<T>::InvalidJson);
            // check the signer is one of the manager or a member of the committee
            let json: Vec<u8> =
                Settings::<T>::get("underwriterssubmission".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            let mut signingtype = 0;
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                    signingtype = 1;
                }
            }
            let operators = json_get_complexarray(json, "committee".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let operator = json_get_arrayvalue(operators.clone(), x);
                if operator.is_empty() {
                    break;
                }
                let operatorvec = bs58::decode(operator).into_vec().unwrap();
                let accountidoperator = T::AccountId::decode(&mut &operatorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if accountidoperator == signer {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 3;
                    }
                }
                x += 1;
            }
            ensure!(
                flag == 1,
                Error::<T>::SignerIsNotAuthorizedForUnderwriterSubmissionOrRemoval
            );

            //Check if Underwriter not already stored on chain
            ensure!(
                !Underwriters::<T>::contains_key(&underwriter_account),
                Error::<T>::UnderwriterAlreadyPresent
            );

            // check for name
            let name = json_get_value(info.clone(), "name".as_bytes().to_vec());
            ensure!(name.len() >= 3, Error::<T>::UnderwriterNameTooShort);

            // check Website
            let website = json_get_value(info.clone(), "website".as_bytes().to_vec());
            ensure!(website.len() >= 10, Error::<T>::UnderwriterWebSiteTooShort);
            ensure!(website.len() <= 64, Error::<T>::UnderwriterWebSiteTooLong);
            ensure!(validate_weburl(website), Error::<T>::InvalidWebsite);

            // check documents
            let ipfsdocs = json_get_complexarray(info.clone(), "ipfsdocs".as_bytes().to_vec());
            if ipfsdocs.len() > 2 {
                let mut x = 0;
                loop {
                    let w = json_get_recordvalue(ipfsdocs.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    let description = json_get_value(w.clone(), "description".as_bytes().to_vec());
                    ensure!(
                        description.len() > 5,
                        Error::<T>::UnderwriterDocumentDescriptionTooShort
                    );
                    let ipfsaddress = json_get_value(w.clone(), "ipfsaddress".as_bytes().to_vec());
                    ensure!(
                        ipfsaddress.len() > 20,
                        Error::<T>::UnderwriterDocumentIpfsAddressTooShort
                    );
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::UnderwriterMissingDocuments);
            }
            // store the underwrited data
            Underwriters::<T>::insert(underwriter_account.clone(), info.clone());

            // Generate event
            Self::deposit_event(Event::UnderwriterCreated(underwriter_account, info));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Destroy an Underwriter
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn undwerwriter_destroy(
            origin: OriginFor<T>,
            underwriter_account: T::AccountId,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // verify the underwriter  exists
            ensure!(
                Underwriters::<T>::contains_key(&underwriter_account),
                Error::<T>::UnderwriterAccountNotFound
            );

            // check the signer is one of the manager or a member of the committee
            let json: Vec<u8> =
                Settings::<T>::get("underwriterssubmission".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            let mut signingtype = 0;
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                    signingtype = 1;
                }
            }
            let operators = json_get_complexarray(json, "committee".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let operator = json_get_arrayvalue(operators.clone(), x);
                if operator.is_empty() {
                    break;
                }
                let operatorvec = bs58::decode(operator).into_vec().unwrap();
                let accountidoperator = T::AccountId::decode(&mut &operatorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if accountidoperator == signer {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 3;
                    }
                }
                x += 1;
            }
            ensure!(
                flag == 1,
                Error::<T>::SignerIsNotAuthorizedForUnderwriterSubmissionOrRemoval
            );

            // Remove the underwriter
            Underwriters::<T>::take(underwriter_account.clone());
            // Generate event
            Self::deposit_event(Event::UnderwriterDestroyed(underwriter_account));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Create an Insurer
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn insurer_create(
            origin: OriginFor<T>,
            insurer_account: T::AccountId,
            info: Vec<u8>,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // check for a valid json structure
            ensure!(json_check_validity(info.clone()), Error::<T>::InvalidJson);
            ensure!(
                Settings::<T>::contains_key("insurersubmission".as_bytes().to_vec()),
                Error::<T>::InsurerSettingIsMissing
            );
            // check the signer is one of the manager or a member of the committee
            let json: Vec<u8> =
                Settings::<T>::get("insurersubmission".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            let mut signingtype = 0;
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                    signingtype = 1;
                }
            }
            let operators = json_get_complexarray(json, "committee".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let operator = json_get_arrayvalue(operators.clone(), x);
                if operator.is_empty() {
                    break;
                }
                let operatorvec = bs58::decode(operator).into_vec().unwrap();
                let accountidoperator = T::AccountId::decode(&mut &operatorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if accountidoperator == signer {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 3;
                    }
                }
                x += 1;
            }
            ensure!(
                flag == 1,
                Error::<T>::SignerIsNotAuthorizedForInsurerSubmissionOrRemoval
            );
            //Check if Insurer not already stored on chain
            ensure!(
                !Insurers::<T>::contains_key(&insurer_account),
                Error::<T>::InsurerAlreadyPresent
            );
            // check for name
            let name = json_get_value(info.clone(), "name".as_bytes().to_vec());
            ensure!(name.len() >= 3, Error::<T>::InsurerNameTooShort);

            // check Website
            let website = json_get_value(info.clone(), "website".as_bytes().to_vec());
            ensure!(website.len() >= 10, Error::<T>::InsurerWebSiteTooShort);
            ensure!(website.len() <= 64, Error::<T>::UnderwriterWebSiteTooLong);
            ensure!(validate_weburl(website), Error::<T>::InvalidWebsite);
            // Check infodocs
            let infodocs = json_get_value(info.clone(), "ipfsdocs".as_bytes().to_vec());
            ensure!(
                !infodocs.is_empty(),
                Error::<T>::MissingInsurerInfoDocuments
            );
            // check documents
            let ipfsdocs = json_get_complexarray(info.clone(), "ipfsdocs".as_bytes().to_vec());
            if ipfsdocs.len() > 2 {
                let mut x = 0;
                loop {
                    let w = json_get_recordvalue(ipfsdocs.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    let description = json_get_value(w.clone(), "description".as_bytes().to_vec());
                    ensure!(
                        description.len() > 5,
                        Error::<T>::InsurerDocumentDescriptionTooShort
                    );
                    let ipfsaddress = json_get_value(w.clone(), "ipfsaddress".as_bytes().to_vec());
                    ensure!(
                        ipfsaddress.len() > 20,
                        Error::<T>::InsurerDocumentIpfsAddressTooShort
                    );
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::InsurerMissingDocuments);
            }
            Insurers::<T>::insert(insurer_account.clone(), info.clone());
            // Generate event
            Self::deposit_event(Event::InsurerCreated(insurer_account, info));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Destroy an Insurer
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn insurer_destroy(
            origin: OriginFor<T>,
            insurer_account: T::AccountId,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // verify the underwriter  exists
            ensure!(
                Underwriters::<T>::contains_key(&insurer_account),
                Error::<T>::InsurerAccountNotFound
            );

            // check the signer is one of the manager or a member of the committee
            let json: Vec<u8> =
                Settings::<T>::get("insurerssubmission".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            let mut signingtype = 0;
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                    signingtype = 1;
                }
            }
            let operators = json_get_complexarray(json, "committee".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let operator = json_get_arrayvalue(operators.clone(), x);
                if operator.is_empty() {
                    break;
                }
                let operatorvec = bs58::decode(operator).into_vec().unwrap();
                let accountidoperator = T::AccountId::decode(&mut &operatorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if accountidoperator == signer {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 3;
                    }
                }
                x += 1;
            }
            ensure!(
                flag == 1,
                Error::<T>::SignerIsNotAuthorizedForInsurerSubmissionOrRemoval
            );
            // Remove the Insurer
            Insurers::<T>::take(insurer_account.clone());
            // Generate event
            Self::deposit_event(Event::InsurerDestroyed(insurer_account));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Create an Insurance - Initially as proposal, it's confirmed once signed and the premium paid from the payer
        /// {"bondid":xxx,"maxcoverage":xxxx,"payer":"xxxxxxxxx","beneficiary":"xxxxoptionalxxxx","premium":xxxxxx,"ipfsdocs":"xxxxxx"}
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn insurance_create(origin: OriginFor<T>, uid: u32, info: Vec<u8>) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            //Get current reserve Balance
            let reserves = InsurerReserves::<T>::get(signer.clone());
            //Using a key "insuranceminreserve", gets the configuration in Settings
            let settings_reserve: Vec<u8> =
                Settings::<T>::get("insuranceminreserve".as_bytes().to_vec()).unwrap();
            //Second key "reserve" gets the reserve minimum required value
            let reserve = json_get_value(settings_reserve, "reserve".as_bytes().to_vec());
            let reserve_min = vecu8_to_u128(reserve);
            ensure!(reserves >= reserve_min, Error::<T>::BelowMinimumReserve);
            // check that the insurer has a reserve
            ensure!(
                InsurerReserves::<T>::contains_key(signer.clone()),
                Error::<T>::ReserveNotFound
            );
            // check for a valid json structure
            ensure!(json_check_validity(info.clone()), Error::<T>::InvalidJson);
            // check the signer is one of the insurers
            ensure!(
                Insurers::<T>::contains_key(&signer),
                Error::<T>::SignerIsNotInsurer
            );
            // check for bondid
            let bondid = json_get_value(info.clone(), "bondid".as_bytes().to_vec());
            ensure!(!bondid.is_empty(), Error::<T>::BondIdIsWrongCannotBeZero);
            let bondidv = vecu8_to_u32(bondid);
            ensure!(
                !Bonds::<T>::contains_key(&bondidv),
                Error::<T>::BondsIdNotFound
            );
            // check max coverage
            let maxcoverage = json_get_value(info.clone(), "maxcoverage".as_bytes().to_vec());
            ensure!(!maxcoverage.is_empty(), Error::<T>::MaxCoverageCannotBeZero);
            let maxcoveragev = vecu8_to_u128(maxcoverage);
            ensure!(maxcoveragev > 0, Error::<T>::MaxCoverageCannotBeZero);
            // check payer account
            let payer = json_get_value(info.clone(), "payer".as_bytes().to_vec());
            ensure!(payer.len() == 48, Error::<T>::PayerAccountIsWrong);
            // check beneficiary account
            let beneficiary = json_get_value(info.clone(), "beneficiary".as_bytes().to_vec());
            ensure!(
                beneficiary.len() == 48,
                Error::<T>::BeneficiaryAccountIsWrong
            );
            // check premium amount
            let premium = json_get_value(info.clone(), "premium".as_bytes().to_vec());
            ensure!(
                !premium.is_empty(),
                Error::<T>::InsurancePremiumCannotBeZero
            );
            let premiumv = vecu8_to_u128(premium);
            ensure!(premiumv > 0, Error::<T>::InsurancePremiumCannotBeZero);
            // check that the reserve staken covers the risk covered including the new insurance
            let reserveamount = InsurerReserves::<T>::get(signer.clone());
            let mut riskcovered: u128 = 0;
            if InsurerRisksCovered::<T>::contains_key(&signer) {
                riskcovered = InsurerRisksCovered::<T>::get(&signer);
            }
            ensure!(
                reserveamount > riskcovered + maxcoveragev,
                Error::<T>::InsufficientReserve
            );
            // Check document
            let ipfsdocs = json_get_value(info.clone(), "ipfsdocs".as_bytes().to_vec());
            ensure!(
                !ipfsdocs.is_empty(),
                Error::<T>::MissingInsurerInfoDocuments
            );
            // check documents
            let ipfsdocs = json_get_complexarray(info.clone(), "ipfsdocs".as_bytes().to_vec());
            if ipfsdocs.len() > 2 {
                let mut x = 0;
                loop {
                    let w = json_get_recordvalue(ipfsdocs.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    let description = json_get_value(w.clone(), "description".as_bytes().to_vec());
                    ensure!(
                        description.len() > 5,
                        Error::<T>::InsurerDocumentDescriptionTooShort
                    );
                    let ipfsaddress = json_get_value(w.clone(), "ipfsaddress".as_bytes().to_vec());
                    ensure!(
                        ipfsaddress.len() > 20,
                        Error::<T>::InsurerDocumentIpfsAddressTooShort
                    );
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::InsurerMissingDocuments);
            }
            //check insurance Id is not already present
            ensure!(
                !Insurances::<T>::contains_key(signer.clone(), &uid),
                Error::<T>::InsuranceIdAlreadyPresent
            );
            // update insurance Risk Covered
            InsurerRisksCovered::<T>::try_mutate(&signer, |risk| -> DispatchResult {
                let total_risk = risk.checked_add(maxcoveragev).ok_or(Error::<T>::Overflow)?;
                *risk = total_risk;
                Ok(())
            })?;
            // Store insurance on chain ready to be signed from the counterpart
            Insurances::<T>::insert(signer.clone(), uid, info.clone());
            // Generate event
            Self::deposit_event(Event::InsuranceCreated(signer, uid, info));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Sign an Insurance transferring the premium to the insurer account anyone can pay the premium
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn insurance_sign(
            origin: OriginFor<T>,
            insurer_account: T::AccountId,
            uid: u32,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // verify the insurance existance
            ensure!(
                Insurances::<T>::contains_key(insurer_account.clone(), uid),
                Error::<T>::InsuranceNotFound
            );
            // verify not already signed
            ensure!(
                !InsurancesSigned::<T>::contains_key(insurer_account.clone(), uid),
                Error::<T>::InsuranceAlreadySigned
            );
            // store the signature
            InsurancesSigned::<T>::insert(insurer_account.clone(), uid, signer.clone());
            // get defaul stable coin
            ensure!(
                Settings::<T>::contains_key("stablecoin".as_bytes().to_vec()),
                Error::<T>::MissingStableCoinConfiguration
            );
            let stablecoin = Settings::<T>::get("stablecoin".as_bytes().to_vec()).unwrap();
            let stablecoinv = json_get_value(stablecoin, "tokenid".as_bytes().to_vec());
            let tokenid = vecu8_to_u32(stablecoinv);
            // transfer the premium
            let info = Insurances::<T>::get(insurer_account.clone(), uid).unwrap();
            let premium = json_get_value(info.clone(), "premium".as_bytes().to_vec());
            let premiumv = vecu8_to_u128(premium);
            pallet_assets::Pallet::<T>::transfer(
                RawOrigin::Signed(signer.clone()).into(),
                tokenid,
                T::Lookup::unlookup(insurer_account.clone()),
                premiumv.into(),
            )
            .unwrap();

            // Generate event
            Self::deposit_event(Event::InsuranceSigned(insurer_account, uid, signer));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Destroy an Insurance, only the original creator can remove if the insurance is not yet signed
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn insurance_destroy(origin: OriginFor<T>, uid: u32) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // verify the insurance existance
            ensure!(
                Insurances::<T>::contains_key(signer.clone(), uid),
                Error::<T>::InsuranceNotFound
            );
            // check that the insurance is not already counter-signed and paid
            ensure!(
                !InsurancesSigned::<T>::contains_key(signer.clone(), uid),
                Error::<T>::InsuranceAlreadySigned
            );
            // get the insurance value
            let info = Insurances::<T>::get(signer.clone(), uid).unwrap();
            let maxcoverage = json_get_value(info, "maxcoverage".as_bytes().to_vec());
            let maxcoveragev = vecu8_to_u128(maxcoverage);
            // update total risk
            InsurerRisksCovered::<T>::try_mutate(&signer, |risk| -> DispatchResult {
                let total_risk = risk
                    .checked_sub(maxcoveragev)
                    .ok_or(Error::<T>::Underflow)?;
                *risk = total_risk;
                Ok(())
            })?;
            // Remove the Insurance
            Insurances::<T>::take(signer.clone(), uid);
            // Generate event
            Self::deposit_event(Event::InsuranceDestroyed(signer, uid));
            // Return a successful DispatchResult
            Ok(())
        }

        /// Create a lawyer
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn lawyer_create(
            origin: OriginFor<T>,
            lawyer_account: T::AccountId,
            info: Vec<u8>,
        ) -> DispatchResult {
            // check for signed transaction
            let signer = ensure_signed(origin)?;
            // check the signer is one of the manager or a member of the committee
            let json: Vec<u8> =
                Settings::<T>::get("lawyerssubmission".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            let mut signingtype = 0;
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                    signingtype = 1;
                }
            }
            let operators = json_get_complexarray(json, "committee".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let operator = json_get_arrayvalue(operators.clone(), x);
                if operator.is_empty() {
                    break;
                }
                let operatorvec = bs58::decode(operator).into_vec().unwrap();
                let accountidoperator = T::AccountId::decode(&mut &operatorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if accountidoperator == signer {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 3;
                    }
                }
                x += 1;
            }
            ensure!(
                flag == 1,
                Error::<T>::SignerIsNotAuthorizedForSubmissionOrRemoval
            );
            // check for a valid json structure
            ensure!(json_check_validity(info.clone()), Error::<T>::InvalidJson);
            //Check if lawyer not already stored on chain
            ensure!(
                !Lawyers::<T>::contains_key(&lawyer_account),
                Error::<T>::AlreadyPresent
            );
            // check for name
            let name = json_get_value(info.clone(), "name".as_bytes().to_vec());
            ensure!(name.len() >= 3, Error::<T>::NameTooShort);
            // check Website
            let website = json_get_value(info.clone(), "website".as_bytes().to_vec());
            ensure!(website.len() >= 10, Error::<T>::WebSiteTooShort);
            ensure!(website.len() <= 64, Error::<T>::WebSiteTooLong);
            ensure!(validate_weburl(website), Error::<T>::InvalidWebsite);
            // Check for documents
            let ipfsdocs = json_get_value(info.clone(), "ipfsdocs".as_bytes().to_vec());
            ensure!(!ipfsdocs.is_empty(), Error::<T>::LawyerMissingDocuments);
            // check documents
            let ipfsdocs = json_get_complexarray(info.clone(), "ipfsdocs".as_bytes().to_vec());
            if ipfsdocs.len() > 2 {
                let mut x = 0;
                loop {
                    let w = json_get_recordvalue(ipfsdocs.clone(), x);
                    if w.is_empty() {
                        break;
                    }
                    let description = json_get_value(w.clone(), "description".as_bytes().to_vec());
                    ensure!(
                        description.len() > 5,
                        Error::<T>::LawyerDocumentDescriptionTooShort
                    );
                    let ipfsaddress = json_get_value(w.clone(), "ipfsaddress".as_bytes().to_vec());
                    ensure!(
                        ipfsaddress.len() > 20,
                        Error::<T>::LawyerDocumentIpfsAddressTooShort
                    );
                    x += 1;
                }
                ensure!(x > 0, Error::<T>::LawyerMissingDocuments);
            }
            // store the lawyer on chain
            Lawyers::<T>::insert(lawyer_account.clone(), info.clone());
            // Generate event LawyerCreated
            Self::deposit_event(Event::LawyerCreated(lawyer_account, info));
            // Return a successful DispatchResult
            Ok(())
        }

        /// Destroy a lawyer
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn lawyer_destroy(
            origin: OriginFor<T>,
            lawyer_account: T::AccountId,
        ) -> DispatchResult {
            let signer = ensure_signed(origin)?;
            // verify the lawyer  exists
            ensure!(
                Lawyers::<T>::contains_key(&lawyer_account),
                Error::<T>::AccountNotFound
            );

            // check the signer is one of the manager or a member of the committee
            let json: Vec<u8> =
                Settings::<T>::get("lawyerssubmission".as_bytes().to_vec()).unwrap();
            let mut flag = 0;
            let mut signingtype = 0;
            let manager = json_get_value(json.clone(), "manager".as_bytes().to_vec());
            if !manager.is_empty() {
                let managervec = bs58::decode(manager).into_vec().unwrap();
                let accountidmanager = T::AccountId::decode(&mut &managervec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if signer == accountidmanager {
                    flag = 1;
                    signingtype = 1;
                }
            }
            let operators = json_get_complexarray(json, "committee".as_bytes().to_vec());
            let mut x = 0;
            loop {
                let operator = json_get_arrayvalue(operators.clone(), x);
                if operator.is_empty() {
                    break;
                }
                let operatorvec = bs58::decode(operator).into_vec().unwrap();
                let accountidoperator = T::AccountId::decode(&mut &operatorvec[1..33])
                    .map_err(|_| Error::<T>::InvalidJson)?;
                if accountidoperator == signer {
                    flag = 1;
                    if signingtype == 0 {
                        signingtype = 3;
                    }
                }
                x += 1;
            }
            ensure!(
                flag == 1,
                Error::<T>::SignerIsNotAuthorizedForSubmissionOrRemoval
            );

            // Remove the lawyer
            Lawyers::<T>::take(lawyer_account.clone());
            // Generate event
            Self::deposit_event(Event::LawyerDestroyed(lawyer_account));
            // Return a successful DispatchResult
            Ok(())
        }

        /// Create Interbank Rate
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn interbankrate_create(
            origin: OriginFor<T>,
            country_code: Vec<u8>,
            date: Vec<u8>,
            rate: u32,
        ) -> DispatchResult {
            // check the transaction is signed from the super user
            ensure_root(origin)?;
            // check country code
            ensure!(
                IsoCountries::<T>::contains_key(&country_code),
                Error::<T>::CountryCodeNotFound
            );
            // check for the date validity
            ensure!(validate_date(&date), Error::<T>::InvalidDateFormat);
            // check Interbank rate does not exists
            ensure!(
                !InterbankRates::<T>::contains_key(&country_code, &date),
                Error::<T>::InterbankRateAlreadyPresent
            );
            // Store Interbank info an integer considering 2 decimals
            InterbankRates::<T>::insert(country_code.clone(), date.clone(), rate);
            // Generate event
            Self::deposit_event(Event::InterbankRateCreated(country_code, date));
            // Return a successful DispatchResult
            Ok(())
        }

        /// Destroy Interbank Rate
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn interbankrate_destroy(
            origin: OriginFor<T>,
            country_code: Vec<u8>,
            date: Vec<u8>,
        ) -> DispatchResult {
            // check the transaction is signed from the super user
            ensure_root(origin)?;
            // check country
            ensure!(
                IsoCountries::<T>::contains_key(&country_code),
                Error::<T>::CountryCodeNotFound
            );
            // Remove Interbank info
            InterbankRates::<T>::take(country_code.clone(), date.clone());
            // Generate event
            Self::deposit_event(Event::InterbankRateDestroyed(country_code, date));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Create Inflation Rate
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn inflationrate_create(
            origin: OriginFor<T>,
            country_code: Vec<u8>,
            date: Vec<u8>,
            rate: u32,
        ) -> DispatchResult {
            // check the transaction is signed from the super user
            ensure_root(origin)?;
            // check country
            ensure!(
                IsoCountries::<T>::contains_key(&country_code),
                Error::<T>::CountryCodeNotFound
            );
            // check for the date validity
            ensure!(validate_date(&date), Error::<T>::InvalidDateFormat);
            // check inflation code does not exists
            ensure!(
                !InflationRates::<T>::contains_key(&country_code, &date),
                Error::<T>::InflationRateAlreadyPresent
            );
            // Store inflation rate info
            InflationRates::<T>::insert(country_code.clone(), date.clone(), rate);
            // Generate event
            Self::deposit_event(Event::InflationRateCreated(country_code, date));
            // Return a successful DispatchResult
            Ok(())
        }

        /// Remove Inflation Rate
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn inflationrate_destroy(
            origin: OriginFor<T>,
            country_code: Vec<u8>,
            date: Vec<u8>,
        ) -> DispatchResult {
            // check the transaction is signed from the super user
            ensure_root(origin)?;
            // check country
            ensure!(
                IsoCountries::<T>::contains_key(&country_code),
                Error::<T>::CountryCodeNotFound
            );
            // remove inflation rate info
            InflationRates::<T>::take(country_code.clone(), date.clone());
            // Generate event
            Self::deposit_event(Event::InflationRateDestroyed(country_code, date));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Stake Reserves for insurers
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn insurance_reserve_stake(origin: OriginFor<T>, amount: Balance) -> DispatchResult {
            // check signature of insurer
            let signer = ensure_signed(origin)?;
            // check it's an approved insurer
            ensure!(
                Insurers::<T>::contains_key(signer.clone()),
                Error::<T>::InsurerAccountNotFound
            );
            ensure!(
                Settings::<T>::contains_key("insurancereserve".as_bytes().to_vec()),
                Error::<T>::InsuranceReserveAccountNotSet
            );
            let json: Vec<u8> = Settings::<T>::get("insurancereserve".as_bytes().to_vec()).unwrap();
            let account = json_get_value(json.clone(), "account".as_bytes().to_vec());
            let accountvec = bs58::decode(account).into_vec().unwrap();
            let accountid = T::AccountId::decode(&mut &accountvec[1..33])
                .map_err(|_| Error::<T>::InvalidJson)?;
            // get defaul stable coin
            ensure!(
                Settings::<T>::contains_key("stablecoin".as_bytes().to_vec()),
                Error::<T>::MissingStableCoinConfiguration
            );
            let stablecoin = Settings::<T>::get("stablecoin".as_bytes().to_vec()).unwrap();
            let stablecoinv = json_get_value(stablecoin, "tokenid".as_bytes().to_vec());
            let tokenid = vecu8_to_u32(stablecoinv);
            //transfer the amount to the reserve account
            pallet_assets::Pallet::<T>::transfer(
                RawOrigin::Signed(signer.clone()).into(),
                tokenid,
                T::Lookup::unlookup(accountid.clone()),
                amount.into(),
            )
            .unwrap();
            // update total reserve for the insurances
            match InsurerReserves::<T>::contains_key(signer.clone()) {
                true => {
                    let current_reserve = InsurerReserves::<T>::take(signer.clone());
                    let new_reserve = current_reserve.checked_add(amount.into()).unwrap();
                    InsurerReserves::<T>::insert(signer.clone(), new_reserve);
                }
                false => {
                    let deposit_into: u128 = amount.into();
                    InsurerReserves::<T>::insert(signer.clone(), deposit_into);
                }
            }
            // emit event
            Self::deposit_event(Event::InsuranceFundStaken(signer, amount));
            Ok(())
        }

        ///Unstake Reserves for insurers
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn insurance_reserve_unstake(origin: OriginFor<T>, amount: Balance) -> DispatchResult {
            // check the transaction is signed
            let signer = ensure_signed(origin)?;
            // check for an available reserve
            ensure!(
                InsurerReserves::<T>::contains_key(signer.clone()),
                Error::<T>::ReserveNotFound
            );
            // check the signer is an insurer
            ensure!(
                Insurers::<T>::contains_key(signer.clone()),
                Error::<T>::InsurerAccountNotFound
            );
            //check the amount is <= the available reserve
            let reserve = Insurers::<T>::get(signer.clone()).unwrap();
            let reservev = vecu8_to_u128(reserve);
            ensure!(reservev >= amount, Error::<T>::CurrentReserveIsNotEnough);
            // check that the reserve - amount > Insurance Coverage
            let reserveamount = InsurerReserves::<T>::get(signer.clone());
            let mut riskcovered: u128 = 0;
            if InsurerRisksCovered::<T>::contains_key(&signer) {
                riskcovered = InsurerRisksCovered::<T>::get(&signer);
            }
            ensure!(
                reserveamount - amount > riskcovered,
                Error::<T>::InsufficientReserve
            );
            ensure!(
                Settings::<T>::contains_key("insurancereserve".as_bytes().to_vec()),
                Error::<T>::InsuranceReserveAccountNotSet
            );
            let json: Vec<u8> = Settings::<T>::get("insurancereserve".as_bytes().to_vec()).unwrap();
            let account = json_get_value(json.clone(), "account".as_bytes().to_vec());
            let accountvec = bs58::decode(account).into_vec().unwrap();
            let accountid = T::AccountId::decode(&mut &accountvec[1..33])
                .map_err(|_| Error::<T>::InvalidJson)?;
            // get defaul stable coin
            ensure!(
                Settings::<T>::contains_key("stablecoin".as_bytes().to_vec()),
                Error::<T>::MissingStableCoinConfiguration
            );
            let stablecoin = Settings::<T>::get("stablecoin".as_bytes().to_vec()).unwrap();
            let stablecoinv = json_get_value(stablecoin, "tokenid".as_bytes().to_vec());
            let tokenid = vecu8_to_u32(stablecoinv);
            //transfer the amount to the reserve account
            pallet_assets::Pallet::<T>::transfer(
                RawOrigin::Signed(accountid.clone()).into(),
                tokenid,
                T::Lookup::unlookup(signer.clone()),
                amount.into(),
            )
            .unwrap();
            // reduce the counter of the reserve
            //Retrieve reserve from InsurerReserves double map
            let current_reserves = InsurerReserves::<T>::get(signer.clone());
            //Current reserves if the withdrawal is done
            let new_reserve: u128 = current_reserves.checked_sub(amount.into()).unwrap();
            InsurerReserves::<T>::take(signer.clone());
            InsurerReserves::<T>::insert(signer.clone(), new_reserve);
            // Emit Event for unstaken
            Self::deposit_event(Event::InsuranceFundUnstaken(signer, amount));
            Ok(())
        }
        /*
        ///Create new order book entry for sale or purchase
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn order_book_create(origin : OriginFor<T>, _uid: u32,_info: Vec<u8>) -> DispatchResult {
            // check the transaction is signed
            let _signer = ensure_signed(origin)?;
            // Emit Event for new book order
            //Self::deposit_event(Event::InsuranceFundUnstaken(signer,amount));
            Ok(())
        }*/
    }
}

// function to validate a json string for no/std. It does not allocate of memory
fn json_check_validity(j: Vec<u8>) -> bool {
    // minimum lenght of 2
    if j.len() < 2 {
        return false;
    }
    // checks star/end with {}
    if *j.get(0).unwrap() == b'{' && *j.last().unwrap() != b'}' {
        return false;
    }
    // checks start/end with []
    if *j.get(0).unwrap() == b'[' && *j.last().unwrap() != b']' {
        return false;
    }
    // check that the start is { or [
    if *j.get(0).unwrap() != b'{' && *j.get(0).unwrap() != b'[' {
        return false;
    }
    //checks that end is } or ]
    if *j.last().unwrap() != b'}' && *j.last().unwrap() != b']' {
        return false;
    }
    //checks " opening/closing and : as separator between name and values
    let mut s: bool = true;
    let mut d: bool = true;
    let mut pg: bool = true;
    let mut ps: bool = true;
    let mut bp = b' ';
    for b in j {
        if b == b'[' && s {
            ps = false;
        }
        if b == b']' && s && !ps {
            ps = true;
        }

        if b == b'{' && s {
            pg = false;
        }
        if b == b'}' && s && !pg {
            pg = true;
        }

        if b == b'"' && s && bp != b'\\' {
            s = false;
            bp = b;
            d = false;
            continue;
        }
        if b == b':' && s {
            d = true;
            bp = b;
            continue;
        }
        if b == b'"' && !s && bp != b'\\' {
            s = true;
            bp = b;
            d = true;
            continue;
        }
        bp = b;
    }

    //fields are not closed properly
    if !s {
        return false;
    }
    //fields are not closed properly
    if !d {
        return false;
    }
    //fields are not closed properly
    if !ps {
        return false;
    }
    //fields are not closed properly
    if !pg {
        return false;
    }
    // every ok returns true
    true
}
// function to get record {} from multirecord json structure [{..},{.. }], it returns an empty Vec when the records is not present
fn json_get_recordvalue(ar: Vec<u8>, p: i32) -> Vec<u8> {
    let mut result = Vec::new();
    let mut op = true;
    let mut cn = 0;
    let mut lb = b' ';
    for b in ar {
        if b == b',' && op {
            cn += 1;
            continue;
        }
        if b == b'[' && op && lb != b'\\' {
            continue;
        }
        if b == b']' && op && lb != b'\\' {
            continue;
        }
        if b == b'{' && op && lb != b'\\' {
            op = false;
        }
        if b == b'}' && !op && lb != b'\\' {
            op = true;
        }
        // field found
        if cn == p {
            result.push(b);
        }
        lb = b;
    }
    result
}
// function to get a field value from array field [1,2,3,4,100], it returns an empty Vec when the records is not present
fn json_get_arrayvalue(ar: Vec<u8>, p: i32) -> Vec<u8> {
    let mut result = Vec::new();
    let mut op = true;
    let mut cn = 0;
    let mut lb = b' ';
    for b in ar {
        if b == b',' && op {
            cn += 1;
            continue;
        }
        if b == b'[' && op && lb != b'\\' {
            continue;
        }
        if b == b']' && op && lb != b'\\' {
            continue;
        }
        if b == b'"' && op && lb != b'\\' {
            continue;
        }
        if b == b'"' && op && lb != b'\\' {
            op = false;
        }
        if b == b'"' && !op && lb != b'\\' {
            op = true;
        }
        // field found
        if cn == p {
            result.push(b);
        }
        lb = b;
    }
    result
}

// function to get value of a field for Substrate runtime (no std library and no variable allocation)
fn json_get_value(j: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut k = Vec::new();
    let keyl = key.len();
    let jl = j.len();
    k.push(b'"');
    for xk in 0..keyl {
        k.push(*key.get(xk).unwrap());
    }
    k.push(b'"');
    k.push(b':');
    let kl = k.len();
    for x in 0..jl {
        let mut m = 0;
        if x + kl > jl {
            break;
        }
        for (xx, i) in (x..x + kl).enumerate() {
            if *j.get(i).unwrap() == *k.get(xx).unwrap() {
                m += 1;
            }
        }
        if m == kl {
            let mut lb = b' ';
            let mut op = true;
            let mut os = true;
            for i in x + kl..jl - 1 {
                if *j.get(i).unwrap() == b'[' && op && os {
                    os = false;
                }
                if *j.get(i).unwrap() == b'}' && op && !os {
                    os = true;
                }
                if *j.get(i).unwrap() == b':' && op {
                    continue;
                }
                if *j.get(i).unwrap() == b'"' && op && lb != b'\\' {
                    op = false;
                    continue;
                }
                if *j.get(i).unwrap() == b'"' && !op && lb != b'\\' {
                    break;
                }
                if *j.get(i).unwrap() == b'}' && op {
                    break;
                }
                if *j.get(i).unwrap() == b']' && op {
                    break;
                }
                if *j.get(i).unwrap() == b',' && op && os {
                    break;
                }
                result.push(*j.get(i).unwrap());
                lb = *j.get(i).unwrap();
            }
            break;
        }
    }
    result
}
// function to get value of a field with a complex array like [{....},{.....}] for Substrate runtime (no std library and no variable allocation)
fn json_get_complexarray(j: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut k = Vec::new();
    let keyl = key.len();
    let jl = j.len();
    k.push(b'"');
    for xk in 0..keyl {
        k.push(*key.get(xk).unwrap());
    }
    k.push(b'"');
    k.push(b':');
    let kl = k.len();
    for x in 0..jl {
        let mut m = 0;
        if x + kl > jl {
            break;
        }
        for (xx, i) in (x..x + kl).enumerate() {
            if *j.get(i).unwrap() == *k.get(xx).unwrap() {
                m += 1;
            }
        }
        if m == kl {
            let mut os = true;
            for i in x + kl..jl - 1 {
                if *j.get(i).unwrap() == b'[' && os {
                    os = false;
                }
                result.push(*j.get(i).unwrap());
                if *j.get(i).unwrap() == b']' && !os {
                    break;
                }
            }
            break;
        }
    }
    result
}
// function to convert vec<u8> to u32
fn vecu8_to_u32(v: Vec<u8>) -> u32 {
    let vslice = v.as_slice();
    let vstr = str::from_utf8(vslice).unwrap_or("0");
    let vvalue: u32 = u32::from_str(vstr).unwrap_or(0);
    vvalue
}

// function to convert vec<u8> to u32
fn vecu8_to_u128(v: Vec<u8>) -> u128 {
    let vslice = v.as_slice();
    let vstr = str::from_utf8(vslice).unwrap_or("0");
    let vvalue: u128 = u128::from_str(vstr).unwrap_or(0);
    vvalue
}

// function to validate a phone number
fn validate_phonenumber(phonenumber: Vec<u8>) -> bool {
    // check maximum lenght
    if phonenumber.len() > 23 {
        return false;
    }
    // check admitted bytes
    let mut x = 0;
    for v in phonenumber.clone() {
        if (48..=57).contains(&v) || (v == 43 && x == 0) {
            x += 1;
        } else {
            return false;
        }
    }
    // load international prefixes table
    let mut p: Vec<Vec<u8>> = vec!["972".into()];
    p.push("93".into());
    p.push("355".into());
    p.push("213".into());
    p.push("376".into());
    p.push("244".into());
    p.push("54".into());
    p.push("374".into());
    p.push("297".into());
    p.push("61".into());
    p.push("43".into());
    p.push("994".into());
    p.push("973".into());
    p.push("880".into());
    p.push("375".into());
    p.push("32".into());
    p.push("501".into());
    p.push("229".into());
    p.push("975".into());
    p.push("387".into());
    p.push("267".into());
    p.push("55".into());
    p.push("246".into());
    p.push("359".into());
    p.push("226".into());
    p.push("257".into());
    p.push("855".into());
    p.push("237".into());
    p.push("1".into());
    p.push("238".into());
    p.push("345".into());
    p.push("236".into());
    p.push("235".into());
    p.push("56".into());
    p.push("86".into());
    p.push("61".into());
    p.push("57".into());
    p.push("269".into());
    p.push("242".into());
    p.push("682".into());
    p.push("506".into());
    p.push("385".into());
    p.push("53".into());
    p.push("537".into());
    p.push("420".into());
    p.push("45".into());
    p.push("253".into());
    p.push("593".into());
    p.push("20".into());
    p.push("503".into());
    p.push("240".into());
    p.push("291".into());
    p.push("372".into());
    p.push("251".into());
    p.push("298".into());
    p.push("679".into());
    p.push("358".into());
    p.push("33".into());
    p.push("594".into());
    p.push("689".into());
    p.push("241".into());
    p.push("220".into());
    p.push("995".into());
    p.push("49".into());
    p.push("233".into());
    p.push("350".into());
    p.push("30".into());
    p.push("299".into());
    p.push("590".into());
    p.push("502".into());
    p.push("224".into());
    p.push("245".into());
    p.push("595".into());
    p.push("509".into());
    p.push("504".into());
    p.push("36".into());
    p.push("354".into());
    p.push("91".into());
    p.push("62".into());
    p.push("964".into());
    p.push("353".into());
    p.push("972".into());
    p.push("39".into());
    p.push("81".into());
    p.push("962".into());
    p.push("254".into());
    p.push("686".into());
    p.push("965".into());
    p.push("996".into());
    p.push("371".into());
    p.push("961".into());
    p.push("266".into());
    p.push("231".into());
    p.push("423".into());
    p.push("370".into());
    p.push("352".into());
    p.push("261".into());
    p.push("265".into());
    p.push("60".into());
    p.push("960".into());
    p.push("223".into());
    p.push("356".into());
    p.push("692".into());
    p.push("596".into());
    p.push("222".into());
    p.push("230".into());
    p.push("262".into());
    p.push("52".into());
    p.push("377".into());
    p.push("976".into());
    p.push("382".into());
    p.push("1664".into());
    p.push("212".into());
    p.push("95".into());
    p.push("264".into());
    p.push("674".into());
    p.push("977".into());
    p.push("31".into());
    p.push("599".into());
    p.push("687".into());
    p.push("64".into());
    p.push("505".into());
    p.push("227".into());
    p.push("234".into());
    p.push("683".into());
    p.push("672".into());
    p.push("47".into());
    p.push("968".into());
    p.push("92".into());
    p.push("680".into());
    p.push("507".into());
    p.push("675".into());
    p.push("595".into());
    p.push("51".into());
    p.push("63".into());
    p.push("48".into());
    p.push("351".into());
    p.push("974".into());
    p.push("40".into());
    p.push("250".into());
    p.push("685".into());
    p.push("378".into());
    p.push("966".into());
    p.push("221".into());
    p.push("381".into());
    p.push("248".into());
    p.push("232".into());
    p.push("65".into());
    p.push("421".into());
    p.push("386".into());
    p.push("677".into());
    p.push("27".into());
    p.push("500".into());
    p.push("34".into());
    p.push("94".into());
    p.push("249".into());
    p.push("597".into());
    p.push("268".into());
    p.push("46".into());
    p.push("41".into());
    p.push("992".into());
    p.push("66".into());
    p.push("228".into());
    p.push("690".into());
    p.push("676".into());
    p.push("216".into());
    p.push("90".into());
    p.push("993".into());
    p.push("688".into());
    p.push("256".into());
    p.push("380".into());
    p.push("971".into());
    p.push("44".into());
    p.push("1".into());
    p.push("598".into());
    p.push("998".into());
    p.push("678".into());
    p.push("681".into());
    p.push("967".into());
    p.push("260".into());
    p.push("263".into());
    p.push("591".into());
    p.push("673".into());
    p.push("61".into());
    p.push("243".into());
    p.push("225".into());
    p.push("500".into());
    p.push("44".into());
    p.push("379".into());
    p.push("852".into());
    p.push("98".into());
    p.push("44".into());
    p.push("44".into());
    p.push("850".into());
    p.push("82".into());
    p.push("856".into());
    p.push("218".into());
    p.push("853".into());
    p.push("389".into());
    p.push("691".into());
    p.push("373".into());
    p.push("258".into());
    p.push("970".into());
    p.push("872".into());
    p.push("262".into());
    p.push("7".into());
    p.push("590".into());
    p.push("290".into());
    p.push("590".into());
    p.push("508".into());
    p.push("239".into());
    p.push("252".into());
    p.push("47".into());
    p.push("963".into());
    p.push("886".into());
    p.push("255".into());
    p.push("670".into());
    p.push("58".into());
    p.push("84".into());
    // normalis number
    let mut startpoint = 0;
    if phonenumber[0] == b'0' && phonenumber[1] == b'0' {
        startpoint = 2;
    }
    if phonenumber[0] == b'+' {
        startpoint = 1;
    }
    // create vec for comparison
    let mut pc3: Vec<u8> = vec![phonenumber[startpoint]];
    pc3.push(phonenumber[startpoint + 1]);
    pc3.push(phonenumber[startpoint + 2]);
    let mut pc2: Vec<u8> = vec![phonenumber[startpoint]];
    pc2.push(phonenumber[startpoint + 1]);
    let pc1: Vec<u8> = vec![phonenumber[startpoint]];
    let mut valid = false;
    for xp in p {
        if xp == pc3 || xp == pc2 || xp == pc1 {
            valid = true;
        }
    }
    valid
}
// function to validate an web url return true/false
fn validate_weburl(weburl: Vec<u8>) -> bool {
    let mut valid = false;
    let mut x = 0;
    let mut httpsflag = false;
    let mut httpflag = false;
    let mut startpoint = 0;
    let mut https: Vec<u8> = vec![b'h'];
    https.push(b't');
    https.push(b't');
    https.push(b'p');
    https.push(b's');
    https.push(b':');
    https.push(b'/');
    https.push(b'/');
    let mut http: Vec<u8> = vec![b'h'];
    http.push(b't');
    http.push(b't');
    http.push(b'p');
    http.push(b':');
    http.push(b'/');
    http.push(b'/');
    let mut httpscomp: Vec<u8> = vec![weburl[0]];
    httpscomp.push(weburl[1]);
    httpscomp.push(weburl[2]);
    httpscomp.push(weburl[3]);
    httpscomp.push(weburl[4]);
    httpscomp.push(weburl[5]);
    httpscomp.push(weburl[6]);
    httpscomp.push(weburl[7]);
    let mut httpcomp: Vec<u8> = vec![weburl[0]];
    httpcomp.push(weburl[1]);
    httpcomp.push(weburl[2]);
    httpcomp.push(weburl[3]);
    httpcomp.push(weburl[4]);
    httpcomp.push(weburl[5]);
    httpcomp.push(weburl[6]);
    if https == httpscomp {
        httpsflag = true;
    }
    if http == httpcomp {
        httpflag = true;
    }
    if !httpflag && !httpsflag {
        return false;
    }
    if httpsflag {
        startpoint = 8;
    }
    if httpflag {
        startpoint = 7;
    }
    for c in weburl {
        if x < startpoint {
            x += 1;
            continue;
        }
        // check for allowed chars
        if (32..=95).contains(&c) || (97..=126).contains(&c) {
            valid = true;
        } else {
            valid = false;
            break;
        }
    }
    valid
}

//function to validate YYYY-MM-DD date format
const DASH_AS_BYTE: u8 = 45;

fn validate_date(date_vec: &[u8]) -> bool {
    let str_date = str::from_utf8(date_vec).unwrap();
    // check date length is correct YYYY-MM-DD

    if str_date.len() != 10 {
        return false;
    }
    if date_vec.to_owned()[4] != DASH_AS_BYTE || date_vec.to_owned()[7] != DASH_AS_BYTE {
        return false;
    }

    let year = &str_date[0..=3];
    let month = &str_date[5..=6];
    let day = &str_date[8..=9];
    if !is_year_valid(year) || !is_day_valid(day) || !is_month_valid(month) {
        return false;
    }

    true
}

fn is_year_valid(year: &str) -> bool {
    let year_u16_res = year.parse();

    if year_u16_res.is_err() {
        return false;
    }
    let year_u16: u16 = year_u16_res.unwrap();

    if !(1900..=2100).contains(&year_u16) {
        return false;
    }
    true
}

fn is_day_valid(day: &str) -> bool {
    let day_u8_res = day.parse();
    if day_u8_res.is_err() {
        return false;
    }
    let day_u8: u8 = day_u8_res.unwrap();

    if !(1..=31).contains(&day_u8) {
        return false;
    }
    true
}

fn is_month_valid(month: &str) -> bool {
    let month_u8_res = month.parse();
    if month_u8_res.is_err() {
        return false;
    }
    let month_u8: u8 = month_u8_res.unwrap();

    if !(1..=12).contains(&month_u8) {
        return false;
    }
    true
}
