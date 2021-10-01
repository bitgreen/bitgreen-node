#![cfg_attr(not(feature = "std"), no_std)]

/// Module to manage the Bonds on BitGreen Blockchain

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure, traits::Currency,codec::Decode};
use frame_system::{ensure_root,ensure_signed};
use sp_std::prelude::*;
use core::str;
use core::str::FromStr;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Module configuration
pub trait Config: frame_system::Config {
//pub trait Config: frame_system::Config + Sized {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type Currency: Currency<Self::AccountId>;
}
pub type Balance = u128;

// The runtime storage items
decl_storage! {
	trait Store for Module<T: Config> as bonds {
		// we use a safe crypto hashing with blake2_128
		// Settings configuration, we store json structure with different keys (see the function for details)
		Settings get(fn get_settings): map hasher(blake2_128_concat) Vec<u8> => Option<Vec<u8>>;
        // Know Your client Data
        Kyc get(fn get_kyc): map hasher(blake2_128_concat) T::AccountId => Option<Vec<u8>>;
        KycSignatures get(fn get_kycsignatures): double_map hasher(blake2_128_concat) T::AccountId,hasher(blake2_128_concat) T::AccountId => Option<u32>;
        // Bonds data
        Bonds get(fn get_bond): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        // Standard Iso country code and official name
        IsoCountries get(fn get_iso_country): map hasher(blake2_128_concat) Vec<u8> => Option<Vec<u8>>;
        // Currencies data
        Currencies get(fn get_currency): map hasher(blake2_128_concat) Vec<u8> => Option<Vec<u8>>
	}
}

// We generate events to inform the users of succesfully actions.
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
	    SettingsCreated(Vec<u8>,Vec<u8>),               // New settings configuration has been created
        SettingsDestroyed(Vec<u8>),                     // A settings has been removed
        BondIssued(AccountId,Vec<u8>),                  // Placeholder for account id, to be removed...
        KycStored(AccountId,Vec<u8>),                   // Kyc data stored on chain
        KycSignedforApproval(AccountId,AccountId),      // Kyc has been signed for approval
        IsoCountryCreated(Vec<u8>,Vec<u8>),             // Iso country created
        IsoCountryDestroyed(Vec<u8>),                   // Iso country destroyed
        CurrencyCodeCreated(Vec<u8>,Vec<u8>),           // a currency code has been created
        CurrencyDestroyed(Vec<u8>),                     // Currency code has been destroyed
	}
);

// Errors to inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
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
        /// Committe for underwriters submission is wrong
        UnderwritersSubmissionCommitteeIsWrong,
        /// Manager account for lawyers submission is wrong
        LawyersSubmissionManagerAccountIsWrong,
        /// Committe for lawyers submission is wrong
        LawyersSubmissionCommitteeIsWrong,
        /// Manager account for collateral verification is wrong    
        CollateralVerificationManagerAccountIsWrong,
        /// Committe for collateral verification is wrong
        CollateralVerificationCommitteeIsWrong,
        ///  Account for Hedge fund approval is wrong    
        HedgeFundApprovalManagerAccountIsWrong,
        /// Committe for Hedge fund approval  is wrong
        HedgeFundApprovalCommitteeIsWrong,
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
        /// Kyc phone is too short
        KycPhoneTooShort,
        /// Kyc phone is too long
        KycPhoneTooLong,
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
        /// The signer is not authorized to approve a KYC
        SignerIsNotAuthorizedForKycApproval,
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
        BondInterestRateCannotBeZero,
        /// Bond interest type is wrong, it can be X,F,Z,I only.
        BondInterestTypeIsWrong,
	}
}

// Dispatchable functions allows users to interact with the pallet BOND and invoke state changes.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized
		type Error = Error<T>;
		// Events must be initialized
		fn deposit_event() = default;
		/// Create/change a  settings configuration. Reserved to super user
        /// We have multiple of configuration:
        /// key=="kyc" {"manager":"xxxaccountidxxx","supervisor":"xxxxaccountidxxxx","operators":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}
        /// for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","supervisor":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","operators":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]}
        /// key=="bondapproval" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...],"mandatoryunderwriting":"Y/N","mandatorycreditrating":"Y/N","mandatorylegalopinion":"Y/N"}
        /// for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"],"mandatoryunderwriting":"Y","mandatorycreditrating":"Y","mandatorylegalopinion":"Y"}
        /// key=="underwriterssubmission" {"manager":"xxxaccountidxxx","committee":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}
        /// for example: {"manager":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","committee":["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y","5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"]}
        /// key=="infodocuments" [{"document":"xxxxdescription"},{"document":"xxxxdescription"}]
        /// for example: [{"document":"Profit&Loss Previous year"},{"document":"Board Members/Director List"}]
        #[weight = 1000]
		pub fn create_change_settings(origin, key: Vec<u8>, configuration: Vec<u8>) -> dispatch::DispatchResult {
			// check the request is signed from Super User
			let _sender = ensure_root(origin)?;
			//check configuration length
			ensure!(configuration.len() > 12, Error::<T>::SettingsJsonTooShort); 
            ensure!(configuration.len() < 8192, Error::<T>::SettingsJsonTooLong); 
            // check json validity
			let js=configuration.clone();
			ensure!(json_check_validity(js),Error::<T>::InvalidJson);
            // check for key validity
            ensure!(key=="kyc".as_bytes().to_vec() 
                || key=="bondapproval".as_bytes().to_vec() 
                || key=="underwriterssubmission".as_bytes().to_vec()
                || key=="collateralverification".as_bytes().to_vec()
                || key=="hedgefundapproval".as_bytes().to_vec()
                || key=="infodocuments".as_bytes().to_vec(),
                Error::<T>::SettingsKeyIsWrong);
            // check validity for kyc settings
            if key=="kyc".as_bytes().to_vec() {
                let manager=json_get_value(configuration.clone(),"manager".as_bytes().to_vec());
                ensure!(manager.len()==48 || manager.len()==0, Error::<T>::KycManagerAccountIsWrong);
                let supervisor=json_get_value(configuration.clone(),"supervisor".as_bytes().to_vec());
                ensure!(supervisor.len()==48 || supervisor.len()==0, Error::<T>::KycSupervisorAccountIsWrong);
                let operators=json_get_complexarray(configuration.clone(),"operators".as_bytes().to_vec());
                if operators.len()>2 {
                    let mut x=0;
                    loop {  
                        let w=json_get_recordvalue(operators.clone(),x);
                        if w.len()==0 {
                            break;
                        }
                        x=x+1;
                    }
                    ensure!(x>0,Error::<T>::KycOperatorsNotConfigured);
                }
            }
            // check validity for bond approval settings
            if key=="bondapproval".as_bytes().to_vec() {
                let manager=json_get_value(configuration.clone(),"manager".as_bytes().to_vec());
                ensure!(manager.len()==48 || manager.len()==0, Error::<T>::BondApprovalManagerAccountIsWrong);
                let committee=json_get_complexarray(configuration.clone(),"committee".as_bytes().to_vec());
                let mut x=0;
                if committee.len()>2 {
                    loop {  
                        let w=json_get_recordvalue(committee.clone(),x);
                        if w.len()==0 {
                            break;
                        }
                        x=x+1;
                    }
                }
                ensure!(x>0,Error::<T>::BondApprovalCommitteeIsWrong);
                let mandatoryunderwriting=json_get_value(configuration.clone(),"mandatoryunderwriting".as_bytes().to_vec());
                ensure!(mandatoryunderwriting=="Y".as_bytes().to_vec() || mandatoryunderwriting=="N".as_bytes().to_vec(), Error::<T>::BondApprovalMandatoryUnderwritingIsWrong);
                let mandatorycreditrating=json_get_value(configuration.clone(),"mandatorycreditrating".as_bytes().to_vec());
                ensure!(mandatorycreditrating=="Y".as_bytes().to_vec() || mandatorycreditrating=="N".as_bytes().to_vec(), Error::<T>::BondApprovalMandatoryCreditRatingIsWrong);
                let mandatorylegalopinion=json_get_value(configuration.clone(),"mandatorylegalopinion".as_bytes().to_vec());
                ensure!(mandatorylegalopinion=="Y".as_bytes().to_vec() || mandatorylegalopinion=="N".as_bytes().to_vec(), Error::<T>::BondApprovalMandatoryLegalOpinionIsWrong);
            }
            // check validity for under writers submission settings
            if key=="underwriterssubmission".as_bytes().to_vec() {
                let manager=json_get_value(configuration.clone(),"manager".as_bytes().to_vec());
                ensure!(manager.len()==48 || manager.len()==0, Error::<T>::UnderWritersSubmissionManagerAccountIsWrong);
                let committee=json_get_complexarray(configuration.clone(),"committee".as_bytes().to_vec());
                let mut x=0;
                if committee.len()>2 {
                    loop {  
                        let w=json_get_recordvalue(committee.clone(),x);
                        if w.len()==0 {
                            break;
                        }
                        x=x+1;
                    }
                }
                ensure!(x>0,Error::<T>::UnderwritersSubmissionCommitteeIsWrong);
            }
            // check validity for lawyers submission settings
            if key=="lawyerssubmission".as_bytes().to_vec() {
                let manager=json_get_value(configuration.clone(),"manager".as_bytes().to_vec());
                ensure!(manager.len()==48 || manager.len()==0, Error::<T>::LawyersSubmissionManagerAccountIsWrong);
                let committee=json_get_complexarray(configuration.clone(),"committee".as_bytes().to_vec());
                let mut x=0;
                if committee.len()>2 {
                    loop {  
                        let w=json_get_recordvalue(committee.clone(),x);
                        if w.len()==0 {
                            break;
                        }
                        x=x+1;
                    }
                }
                ensure!(x>0,Error::<T>::LawyersSubmissionCommitteeIsWrong);
            }
            // check validity for collateral verification settings
            if key=="collateralverification".as_bytes().to_vec() {
                let manager=json_get_value(configuration.clone(),"manager".as_bytes().to_vec());
                ensure!(manager.len()==48 || manager.len()==0, Error::<T>::CollateralVerificationManagerAccountIsWrong);
                let committee=json_get_complexarray(configuration.clone(),"committee".as_bytes().to_vec());
                let mut x=0;
                if committee.len()>2 {
                    loop {  
                        let w=json_get_recordvalue(committee.clone(),x);
                        if w.len()==0 {
                            break;
                        }
                        x=x+1;
                    }
                }
                ensure!(x>0,Error::<T>::CollateralVerificationCommitteeIsWrong);
            }
            // check validity for hedge fund approval settings
            if key=="hedgefundapproval".as_bytes().to_vec() {
                let manager=json_get_value(configuration.clone(),"manager".as_bytes().to_vec());
                ensure!(manager.len()==48 || manager.len()==0, Error::<T>::HedgeFundApprovalManagerAccountIsWrong);
                let committee=json_get_complexarray(configuration.clone(),"committee".as_bytes().to_vec());
                let mut x=0;
                if committee.len()>2 {
                    loop {  
                        let w=json_get_recordvalue(committee.clone(),x);
                        if w.len()==0 {
                            break;
                        }
                        x=x+1;
                    }
                }
                ensure!(x>0,Error::<T>::HedgeFundApprovalCommitteeIsWrong);
            }
            // check validity for info documents
            if key=="infodocuments".as_bytes().to_vec() {
                let documents=json_get_complexarray(configuration.clone(),"documents".as_bytes().to_vec());
                let mut x=0;
                if documents.len()>2 {
                    loop {  
                        let w=json_get_recordvalue(documents.clone(),x);
                        if w.len()==0 {
                            break;
                        }
                        x=x+1;
                    }
                }
                ensure!(x>0,Error::<T>::InfoDocumentsIsWrong);
            }
            //store settings on chain
            if Settings::contains_key(&key)==false {
                // Insert settings
                Settings::insert(key.clone(),configuration.clone());
            } else {
                // Replace Settings Data 
                Settings::take(key.clone());
                Settings::insert(key.clone(),configuration.clone());
            }
            // Generate event
			Self::deposit_event(RawEvent::SettingsCreated(key,configuration));
			// Return a successful DispatchResult
			Ok(())
		}
	
        // this function has the purpose to the insert or update data for KYC
        #[weight = 1000]
        pub fn create_change_kyc(origin, accountid: T::AccountId, info: Vec<u8>) -> dispatch::DispatchResult {
            let _signer = ensure_signed(origin)?;
            // TODO check the signer is one of the operators for kyc
            //check info length
            ensure!(info.len() < 8192, Error::<T>::KycInfoIsTooLong); 
            // check json validity
            let js=info.clone();
            ensure!(json_check_validity(js),Error::<T>::InvalidJson);
            // check name
            let name=json_get_value(info.clone(),"name".as_bytes().to_vec());
            ensure!(name.len()>=10,Error::<T>::KycNameTooShort);
            ensure!(name.len()<=64,Error::<T>::KycNameTooLong);
            // check Address
            let address=json_get_value(info.clone(),"address".as_bytes().to_vec());
            ensure!(address.len()>=10,Error::<T>::KycAddressTooShort);
            ensure!(address.len()<=64,Error::<T>::KycAddressTooLong);
            // check Zip code
            let zip=json_get_value(info.clone(),"zip".as_bytes().to_vec());
            ensure!(zip.len()>3,Error::<T>::KycZipCodeTooShort);
            ensure!(zip.len()<=6,Error::<T>::KycZipCodeTooLong);
            // check City
            let city=json_get_value(info.clone(),"city".as_bytes().to_vec());
            ensure!(city.len()>3,Error::<T>::KycCityTooShort);
            ensure!(city.len()<=64,Error::<T>::KycCityTooLong);
            // check State
            let state=json_get_value(info.clone(),"state".as_bytes().to_vec());
            ensure!(state.len()>3,Error::<T>::KycStateTooShort);
            ensure!(state.len()<=64,Error::<T>::KycStateTooLong);
            // check Country
            let country=json_get_value(info.clone(),"country".as_bytes().to_vec());
            ensure!(country.len()>3,Error::<T>::KycCountryTooShort);
            ensure!(country.len()<64,Error::<T>::KycCountryTooLong);
            // check Website
            let website=json_get_value(info.clone(),"website".as_bytes().to_vec());
            ensure!(website.len()>=10,Error::<T>::KycWebSiteTooShort);
            ensure!(website.len()<=64,Error::<T>::KycWebSiteTooLong);
            // TODO url validity
            // check Phone lenght
            let phone=json_get_value(info.clone(),"phone".as_bytes().to_vec());
            ensure!(phone.len()>=10,Error::<T>::KycPhoneTooShort);
            ensure!(phone.len()<=21,Error::<T>::KycPhoneTooLong);
            //TODO check prefix
            let ipfsdocs=json_get_complexarray(info.clone(),"ipfsdocs".as_bytes().to_vec());
            if ipfsdocs.len()>2 {
                let mut x=0;
                loop {  
                    let w=json_get_recordvalue(ipfsdocs.clone(),x);
                    if w.len()==0 {
                        break;
                    }
                    let description=json_get_value(w.clone(),"description".as_bytes().to_vec());
                    ensure!(description.len()>5,Error::<T>::KycDocumentDescriptionTooShort);
                    let ipfsaddress=json_get_value(w.clone(),"ipfsaddress".as_bytes().to_vec());
                    ensure!(ipfsaddress.len()>20,Error::<T>::KycDocumentIpfsAddressTooShort);

                    x=x+1;
                }
                ensure!(x>0,Error::<T>::KycMissingDocuments);
            }
            //store Kyc on chain
            if Kyc::<T>::contains_key(&accountid)==false {
                // Insert kyc
                Kyc::<T>::insert(accountid.clone(),info.clone());
            } else {
                // TODO: check that is not already approved from anybody
                // Replace Kyc Data 
                Kyc::<T>::take(accountid.clone());
                Kyc::<T>::insert(accountid.clone(),info.clone());
            }
            // Generate event
            Self::deposit_event(RawEvent::KycStored(accountid,info));
            // Return a successful DispatchResult
            Ok(())
        }
        #[weight = 1000]
        pub fn kyc_approve(origin, accountid: T::AccountId) -> dispatch::DispatchResult {
            let signer = ensure_signed(origin)?;
            let mut signingtype=0;
            //check id >0
            ensure!(Kyc::<T>::contains_key(&accountid),Error::<T>::KycIdNotFound);
            ensure!(!KycSignatures::<T>::contains_key(&accountid,&signer),Error::<T>::KycSignatureAlreadyPresentrSameSigner);
            // check the signer is one of the operators for kyc
            let json:Vec<u8>=Settings::get("kyc".as_bytes().to_vec()).unwrap();
            let mut flag=0;
            let manager=json_get_value(json.clone(),"manager".as_bytes().to_vec());
            if manager.len()>0 {
                let managervec=bs58::decode(manager).into_vec().unwrap();
                let accountidmanager=T::AccountId::decode(&mut &managervec[1..33]).unwrap_or_default();
                if signer==accountidmanager {
                    flag=1;       
                    signingtype=1;             
                }
            }
            let supervisor=json_get_value(json.clone(),"supervisor".as_bytes().to_vec());
            if supervisor.len()>0 {
                let supervisorvec=bs58::decode(supervisor).into_vec().unwrap();
                let accountidsupervisor=T::AccountId::decode(&mut &supervisorvec[1..33]).unwrap_or_default();
                if signer==accountidsupervisor {
                    flag=1;
                    if signingtype==0 {
                        signingtype=2;             
                    }
                }
            }
            let operators=json_get_complexarray(json.clone(),"operators".as_bytes().to_vec());
            let mut x=0;
            loop {  
                let operator=json_get_arrayvalue(operators.clone(),x);
                if operator.len()==0 {
                    break;
                }
                let operatorvec=bs58::decode(operator).into_vec().unwrap();
                let accountidoperator=T::AccountId::decode(&mut &operatorvec[1..33]).unwrap_or_default();
                if accountidoperator==signer {
                    flag=1;
                    if signingtype==0 {
                        signingtype=3;             
                    }
                }
                x=x+1;
            }
            ensure!(flag==1,Error::<T>::SignerIsNotAuthorizedForKycApproval);
            // write/update signature
            KycSignatures::<T>::insert(accountid.clone(),signer.clone(),1);
            // check for all the approval
            // generate event
            Self::deposit_event(RawEvent::KycSignedforApproval(accountid,signer));
            // Return a successful DispatchResult
            Ok(())
        }  
        // Function to create a new bond subject to approval. The info field is a json structure with the following fields:
        // totalamount: total amount considering 0 decimals
        // currency: is the currency code as from the blockchain storage "Currencies"
        // country: is the the iso conty code as from blockchain storage "IsoCountries"
        // interestrate: is the interest rate expressed in an integer assumin 2 decimals, for example 200 is equivalent to 2.00 %
        // interest type: X=Fixed Rate / F=Floating Rate /Z= Zero Interest/ I= Inflation Linked
        // TODO - Oracle to get Inflation rate
        // for example: 
        // {"totalamount":100000000,
        #[weight = 1000]
        pub fn bond_create(origin, id: u32,info: Vec<u8>) -> dispatch::DispatchResult {
            let signer = ensure_signed(origin)?;
            //check id >0
            ensure!(id>0, Error::<T>::BondIdIsWrongCannotBeZero); 
            ensure!(!Bonds::contains_key(&id),Error::<T>::BondIdAlreadyUsed);
            // check the signer has been subject to KYC approval
            ensure!(Kyc::<T>::contains_key(&signer),Error::<T>::MissingKycForSigner);
            // check the Kyc has been approved
            // TODO check all the type of approvals
            //ensure!(!KycSignatures::contains_key(&signer),Error::<T>::MissingKycApproval);
            // check total amount
            let totalamount=json_get_value(info.clone(),"totalamount".as_bytes().to_vec());
            let totalamountv=vecu8_to_u32(totalamount);
            ensure!(totalamountv>0,Error::<T>::BondTotalAmountCannotBeZero);
            let currency=json_get_value(info.clone(),"currency".as_bytes().to_vec());
            ensure!(Currencies::contains_key(&currency), Error::<T>::CurrencyCodeNotFound);
            let country=json_get_value(info.clone(),"country".as_bytes().to_vec());
            ensure!(IsoCountries::contains_key(&country), Error::<T>::CountryCodeNotFound);
            let country=json_get_value(info.clone(),"country".as_bytes().to_vec());
            ensure!(IsoCountries::contains_key(&country), Error::<T>::CountryCodeNotFound);
            let interestrate=json_get_value(info.clone(),"interestrate".as_bytes().to_vec());
            let interestratev=vecu8_to_u32(interestrate);
            ensure!(interestratev>0,Error::<T>::BondInterestRateCannotBeZero);
            let interestype=json_get_value(info.clone(),"interestype".as_bytes().to_vec());
            ensure!(interestype[0]==b'X' 
                || interestype[0]==b'F'  
                || interestype[0]==b'Z' 
                || interestype[0]==b'I' 
                ,Error::<T>::BondInterestTypeIsWrong);
            Ok(())
        }  
        /// Create a new Iso country code and name
        #[weight = 1000]
        pub fn iso_country_create(origin, countrycode: Vec<u8>, countryname: Vec<u8>) -> dispatch::DispatchResult {
            // check the request is signed from the Super User
            let _sender = ensure_root(origin)?;
            // check country code length == 2
            ensure!(countrycode.len()==2, Error::<T>::WrongLengthCountryCode);
            // check country name length  >= 3
            ensure!(countryname.len()>=3, Error::<T>::CountryNameTooShort);
            // check the country is not alreay present on chain
            ensure!(IsoCountries::contains_key(&countrycode)==false, Error::<T>::CountryCodeAlreadyPresent);
            // store the Iso Country Code and Name
            IsoCountries::insert(countrycode.clone(),countryname.clone());
            // Generate event
            Self::deposit_event(RawEvent::IsoCountryCreated(countrycode,countryname));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Destroy an Iso country code and name
        #[weight = 1000]
        pub fn iso_country_destroy(origin, countrycode: Vec<u8>,) -> dispatch::DispatchResult {
            // check the request is signed from the Super User
            let _sender = ensure_root(origin)?;
            // verify the country code exists
            ensure!(IsoCountries::contains_key(&countrycode)==true, Error::<T>::CountryCodeNotFound);
            // Remove country code
            IsoCountries::take(countrycode.clone());
            // Generate event
            //it can leave orphans, anyway it's a decision of the super user
            Self::deposit_event(RawEvent::IsoCountryDestroyed(countrycode));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Create a new Currency code with name and other info in a json structure
         /// {"name":"Bitcoin","category":"c(rypto)/f(iat)","country":"countryisocode","blockchain":"Ethereum(...)","address":"xxxfor_crypto_currencyxxx"}
         /// for example: {"name":"Bitcoin","category":"c","country":"AE","blockchain":"Bitcoin","address":"not applicable"}
         /// {"name":"American Dollars","category":"f","country":"US","blockchain":"not applicable","address":"not applicable"}
         #[weight = 1000]
         pub fn currency_create(origin, currencycode: Vec<u8>, info: Vec<u8>) -> dispatch::DispatchResult {
             // check the request is signed from the Super User
             let _sender = ensure_root(origin)?;
             // check currency code length is between 3 and 5 bytes
             ensure!((currencycode.len()>=3 && currencycode.len()<=5), Error::<T>::WrongLengthCurrencyCode);
             // check the info field is not longer 1024 bytes
             ensure!((info.len()<=1024), Error::<T>::SizeInfoTooLong);
             // check for a valid json structure
             ensure!(json_check_validity(info.clone()),Error::<T>::InvalidJson);
             // check for name
             let name=json_get_value(info.clone(),"name".as_bytes().to_vec());
             ensure!(name.len()>=3, Error::<T>::CurrencyNameTooShort);
             ensure!(name.len()<=32, Error::<T>::CurrencyNameTooLong);
             // check for type of currency (fiat/crypto)
             let category=json_get_value(info.clone(),"category".as_bytes().to_vec());
             let mut c: Vec<u8>= Vec::new();
             c.push(b'c');
             let mut f: Vec<u8>= Vec::new();
             f.push(b'f');
             ensure!((category==c || category==f),Error::<T>::CurrencyCategoryIswrong);
             // check for the country code in case of Fiat currency
             if category==f {
                 let countrycode=json_get_value(info.clone(),"country".as_bytes().to_vec());
                 ensure!(IsoCountries::contains_key(&countrycode), Error::<T>::CountryCodeNotFound);
             }
             // check for the blockchain in case of Crypto currency
             if category==c {
                 let blockchain=json_get_value(info.clone(),"blockchain".as_bytes().to_vec());
                 ensure!(blockchain.len()>=3, Error::<T>::BlockchainNameTooShort);
                 ensure!(blockchain.len()<=32, Error::<T>::BlockchainNameTooLong);
             }
             // check the currency is not alreay present on chain
             ensure!(!Currencies::contains_key(&currencycode), Error::<T>::CurrencyCodeAlreadyPresent);
             // store the Currency Code and info
             Currencies::insert(currencycode.clone(),info.clone());
             // Generate event
             Self::deposit_event(RawEvent::CurrencyCodeCreated(currencycode,info));
             // Return a successful DispatchResult
             Ok(())
         }
         /// Destroy a currency
         #[weight = 1000]
         pub fn currency_destroy(origin, currencycode: Vec<u8>,) -> dispatch::DispatchResult {
             // check the request is signed from the Super User
             let _sender = ensure_root(origin)?;
             // verify the currency code exists
             ensure!(Currencies::contains_key(&currencycode), Error::<T>::CurrencyCodeNotFound);
             // Remove currency code
             Currencies::take(currencycode.clone());
             // Generate event
             //it can leave orphans, anyway it's a decision of the super user
             Self::deposit_event(RawEvent::CurrencyDestroyed(currencycode));
             // Return a successful DispatchResult
             Ok(())
         }
    }
}
// function to validate a json string for no/std. It does not allocate of memory
fn json_check_validity(j:Vec<u8>) -> bool{	
    // minimum lenght of 2
    if j.len()<2 {
        return false;
    }
    // checks star/end with {}
    if *j.get(0).unwrap()==b'{' && *j.get(j.len()-1).unwrap()!=b'}' {
        return false;
    }
    // checks start/end with []
    if *j.get(0).unwrap()==b'[' && *j.get(j.len()-1).unwrap()!=b']' {
        return false;
    }
    // check that the start is { or [
    if *j.get(0).unwrap()!=b'{' && *j.get(0).unwrap()!=b'[' {
            return false;
    }
    //checks that end is } or ]
    if *j.get(j.len()-1).unwrap()!=b'}' && *j.get(j.len()-1).unwrap()!=b']' {
        return false;
    }
    //checks " opening/closing and : as separator between name and values
    let mut s:bool=true;
    let mut d:bool=true;
    let mut pg:bool=true;
    let mut ps:bool=true;
    let mut bp = b' ';
    for b in j {
        if b==b'[' && s {
            ps=false;
        }
        if b==b']' && s && ps==false {
            ps=true;
        }
        else if b==b']' && s && ps==true {
            ps=false;
        }
        if b==b'{' && s {
            pg=false;
        }
        if b==b'}' && s && pg==false {
            pg=true;
        }
        else if b==b'}' && s && pg==true {
            pg=false;
        }
        if b == b'"' && s && bp != b'\\' {
            s=false;
            bp=b;
            d=false;
            continue;
        }
        if b == b':' && s {
            d=true;
            bp=b;
            continue;
        }
        if b == b'"' && !s && bp != b'\\' {
            s=true;
            bp=b;
            d=true;
            continue;
        }
        bp=b;
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
    // every ok returns true
    return true;
}
// function to get record {} from multirecord json structure [{..},{.. }], it returns an empty Vec when the records is not present
fn json_get_recordvalue(ar:Vec<u8>,p:i32) -> Vec<u8> {
    let mut result=Vec::new();
    let mut op=true;
    let mut cn=0;
    let mut lb=b' ';
    for b in ar {
        if b==b',' && op==true {
            cn=cn+1;
            continue;
        }
        if b==b'[' && op==true && lb!=b'\\' {
            continue;
        }
        if b==b']' && op==true && lb!=b'\\' {
            continue;
        }
        if b==b'{' && op==true && lb!=b'\\' { 
            op=false;
        }
        if b==b'}' && op==false && lb!=b'\\' {
            op=true;
        }
        // field found
        if cn==p {
            result.push(b);
        }
        lb=b.clone();
    }
    return result;
}
// function to get a field value from array field [1,2,3,4,100], it returns an empty Vec when the records is not present
fn json_get_arrayvalue(ar:Vec<u8>,p:i32) -> Vec<u8> {
    let mut result=Vec::new();
    let mut op=true;
    let mut cn=0;
    let mut lb=b' ';
    for b in ar {
        if b==b',' && op==true {
            cn=cn+1;
            continue;
        }
        if b==b'[' && op==true && lb!=b'\\' {
            continue;
        }
        if b==b']' && op==true && lb!=b'\\' {
            continue;
        }
        if b==b'"' && op==true && lb!=b'\\' {
            continue;
        }
        if b==b'"' && op==true && lb!=b'\\' { 
            op=false;
        }
        if b==b'"' && op==false && lb!=b'\\' {
            op=true;
        }
        // field found
        if cn==p {
            result.push(b);
        }
        lb=b.clone();
    }
    return result;
}

// function to get value of a field for Substrate runtime (no std library and no variable allocation)
fn json_get_value(j:Vec<u8>,key:Vec<u8>) -> Vec<u8> {
    let mut result=Vec::new();
    let mut k=Vec::new();
    let keyl = key.len();
    let jl = j.len();
    k.push(b'"');
    for xk in 0..keyl{
        k.push(*key.get(xk).unwrap());
    }
    k.push(b'"');
    k.push(b':');
    let kl = k.len();
    for x in  0..jl {
        let mut m=0;
        let mut xx=0;
        if x+kl>jl {
            break;
        }
        for i in x..x+kl {
            if *j.get(i).unwrap()== *k.get(xx).unwrap() {
                m=m+1;
            }
            xx=xx+1;
        }
        if m==kl{
            let mut lb=b' ';
            let mut op=true;
            let mut os=true;
            for i in x+kl..jl-1 {
                if *j.get(i).unwrap()==b'[' && op==true && os==true{
                    os=false;
                }
                if *j.get(i).unwrap()==b'}' && op==true && os==false{
                    os=true;
                }
                if *j.get(i).unwrap()==b':' && op==true{
                    continue;
                }
                if *j.get(i).unwrap()==b'"' && op==true && lb!=b'\\' {
                    op=false;
                    continue
                }
                if *j.get(i).unwrap()==b'"' && op==false && lb!=b'\\' {
                    break;
                }
                if *j.get(i).unwrap()==b'}' && op==true{
                    break;
                }
                if *j.get(i).unwrap()==b']' && op==true{
                    break;
                }
                if *j.get(i).unwrap()==b',' && op==true && os==true{
                    break;
                }
                result.push(j.get(i).unwrap().clone());
                lb=j.get(i).unwrap().clone();
            }   
            break;
        }
    }
    return result;
}
// function to get value of a field with a complex array like [{....},{.....}] for Substrate runtime (no std library and no variable allocation)
fn json_get_complexarray(j:Vec<u8>,key:Vec<u8>) -> Vec<u8> {
    let mut result=Vec::new();
    let mut k=Vec::new();
    let keyl = key.len();
    let jl = j.len();
    k.push(b'"');
    for xk in 0..keyl{
        k.push(*key.get(xk).unwrap());
    }
    k.push(b'"');
    k.push(b':');
    let kl = k.len();
    for x in  0..jl {
        let mut m=0;
        let mut xx=0;
        if x+kl>jl {
            break;
        }
        for i in x..x+kl {
            if *j.get(i).unwrap()== *k.get(xx).unwrap() {
                m=m+1;
            }
            xx=xx+1;
        }
        if m==kl{
            let mut os=true;
            for i in x+kl..jl-1 {
                if *j.get(i).unwrap()==b'[' && os==true{
                    os=false;
                }
                result.push(j.get(i).unwrap().clone());
                if *j.get(i).unwrap()==b']' && os==false {
                    break;
                }
            }   
            break;
        }
    }
    return result;
}
// function to convert vec<u8> to u32
fn vecu8_to_u32(v: Vec<u8>) -> u32 {
    let vslice = v.as_slice();
    let vstr = str::from_utf8(&vslice).unwrap_or("0");
    let vvalue: u32 = u32::from_str(vstr).unwrap_or(0);
    vvalue
}


