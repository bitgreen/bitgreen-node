#![cfg_attr(not(feature = "std"), no_std)]

/// Module to manage the impact actions on BitGreen Blockchain

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure, traits::Currency};
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
	trait Store for Module<T: Config> as impactactions {
		// we use a safe crypto hashing with blake2_128
		// We store the impact actions configuration 
		ImpactActions get(fn get_impactaction): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        // Categories for impact actions
        ImpactActionsCategories get(fn get_category): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        // Auditor Configuration
        ImpactActionsAuditors get(fn get_auditor): map hasher(blake2_128_concat) T::AccountId => Option<Vec<u8>>;
        // Auditor Configuration
        ImpactActionsOracles get(fn get_oracle): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        // Impact Action Submission
        ImpactActionsSubmissions get(fn get_approval_request): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        //Assigned Auditors
        ImpactActionsSubmissionsAuditors get(fn get_auditor_assigned): double_map hasher(blake2_128_concat) u32, hasher(blake2_128_concat) T::AccountId => Option<u32>;
         //Approval/Refusal of the submissions
        ImpactActionsSubmissionsVotes get(fn get_approval_vote): double_map hasher(blake2_128_concat) u32, hasher(blake2_128_concat)T::AccountId => Option<Vec<u8>>;
        // Proxy Account for assigning auditors
        ImpactActionsProxy get(fn get_proxy_account): map hasher(blake2_128_concat) u32 => Option<T::AccountId>;
	}
}

// We generate events to inform the users of succesfully actions.
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
	    ImpactActionCreated(u32,Vec<u8>),               // New Impact Action configuration has been created
        ImpactActionDestroyed(u32),                     // Impact action configuration has been removed
        ImpactActionProxyCreated(u32),                  // Proxy account created
        ImpactActionProxyDestroyed(u32),                // Proxy account removed
        ImpactActionRequestApproval(AccountId,u32),     // Impact action approval request
        ImpactActionCategoryCreated(u32,Vec<u8>),       // A new category has been created
        ImpactActionCategoryDestroyed(u32),             // A category has been removed
        ImpactActionAuditorCreated(AccountId,Vec<u8>),        // A new auditor has been created
        ImpactActionAuditorDestroyed(AccountId),              // An auditor has been removed
        ImpactActionOracleCreated(u32,Vec<u8>),         // A new oracle has been added
        ImpactActionOracleDestroyed(u32),               // An oracle has been removed
        ImpactActionAuditorAssigned(u32,AccountId,u32),        // Assigned auditor to a request approval with xx max days to complete the auditing
        ImpactActionRequestApprovalVoted(AccountId,u32,Vec<u8>),
	}
);

// Errors to inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Uid cannot be zero
		UidCannotBeZero,
		/// The impact action is already on chain (the same uid)
		DuplicatedImpactAction,
		/// Wrong configuration lenght, it must be > 12 bytes
		TooShortConfigurationLength,
        /// Wrong configuration lenght, it must be <=8192
        TooLongConfigurationLength,
        /// Too short description for the impact action, it must be > 4.
        TooShortDescription,
        /// Too short description for the impact action, it must be <=1024.
        TooLongDescription,
        /// Too short categories for the impact action, it must be >=3.
        TooShortCategories,
        /// Too short categories for the impact action, it must be <=256.
        TooLongCategories,
        /// Invalid start block number, it must  be >0
        InvalidBlockStart,
        /// Invalid end block number, it must  be >0
        InvalidBlockEnd,
        /// Invalid rewards token, it must >=0
        InvalidRewardsToken,
        /// Invalid rewards amount, it must be >0
        InvalidRewardsAmount,
        /// Invalid rewards Oracle, it must be >=0
        InvalidRewardsOracle,
        /// Invalid rewards Auditors, it must be >=0
        InvalidRewardsAuditors,
        /// Invalid slashing amount for Auditors, it must be >=0
        InvalidSlashingAuditors,
        /// Invalid number of maximum errors for an auditor to be revoked
        InvalidMaxErrorsAuditor,
        /// Invalid Json received, check the sintax
        InvalidJson,
        /// Impact action not found
        ImpactActionNotFound,
        /// Category description is too short
        TooShortCategoryDescription,
        /// Category description is too long
        TooLongCategoryDescription,
        /// Category of the impact action is already present with the same id
        DuplicatedCategoryImpactAction,
        /// Category of impact action has not been found
        CategoryImpactActionNotFound,
        /// area field is too short
        TooShortArea,
        /// area field is too long
        TooLongArea,
        /// Minimum Stakes must be >=0
        InvalidStakesMinimum,
        /// Other info is too long, it must be < 1024
        TooLongOtherInfo,
        /// Too short info field
        TooShortInfo,
        /// Too long info field
        TooLongInfo,
        /// Approval already present for the same id
        ImpactActionSubmissionDuplicated,
        /// The auditor id cannot be found
        AuditorImpactActionNotFound,
        /// Oracle account is not valid it should be long 48 bytes
        OracleAccountNotValid,
        /// Other info missing
        OtherInfoMissing,
        /// Oracle Impact action found
        OracleImpactActionNotFound,
        /// Proxy id is already present, remove it before to create again.
        DuplicatedProxyId,
        /// Proxy account not found
        ProxyAccountNotFound,
        /// Impact Action Submission has not been found
        ImpactActionSubmissionNotFound,
        /// Auditor cannot be equal to zero
        AuditorCannotBeZero,
        /// Max days for auditing cannot be equal to zero
        MaxDaysCannotBeZero,
        /// The auditor account is already present, it cannot be duplicated
        DuplicatedImpactActionAuditor,
        /// The signer is not assigned as auditor to this impact action
        SignerNotAssigneAsAuditor,
        /// Vote is not valid, should be Y or N
        VoteIsInvalid,
        /// Other info is too short it must be > 2 bytes
        OtherInfoTooShort,
        /// Other info is too long it must be < 1024 bytes
        OtherInfoTooLong,
        /// The signing account is not a valid proxy for the operation required.
        SigningAccountNotValidProxy,
        /// Number of auditors must be > 0
        NumberofAuditorsCannotBeZero,
        /// Category cannot be zero
        CategoryCannotBeZero,
        /// Category has not been found
        CategoryNotFound,
        /// Field name is too short, it must be > 0 
        FieldNameTooShort,
        /// Field type is wrong, it can be N=Numbers or S=String
        FieldTypeIsWrong,
        /// The mandatory flag can be Y or N only
        FieldMandatoryFlagIsWrong,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized
		type Error = Error<T>;
		// Events must be initialized
		fn deposit_event() = default;
		/// Create a new impact action configuration
        #[weight = 1000]
		pub fn create_impact_action(origin, uid: u32, configuration: Vec<u8>) -> dispatch::DispatchResult {
			// check the request is signed from Super User
			let _sender = ensure_root(origin)?;
			//check configuration length
			ensure!(configuration.len()< 12, Error::<T>::TooShortConfigurationLength); 
            ensure!(configuration.len()> 8192, Error::<T>::TooLongConfigurationLength); 
			// check the id is > 0
			ensure!(uid > 0, Error::<T>::UidCannotBeZero); 
			// check that the uid is not already present
			ensure!(ImpactActions::contains_key(&uid)==false, Error::<T>::DuplicatedImpactAction);
            // check json validity
			let js=configuration.clone();
			ensure!(json_check_validity(js),Error::<T>::InvalidJson);
            // check description
			let jsd=configuration.clone();
			let description=json_get_value(jsd,"description".as_bytes().to_vec());
			ensure!(description.len() >= 4, Error::<T>::TooShortDescription); //check minimum length for the description
			ensure!(description.len() <=1024, Error::<T>::TooLongDescription); //check maximum length for the description
            // check categories
            let jsc=configuration.clone();
            let categories=json_get_value(jsc,"categories".as_bytes().to_vec());
            ensure!(categories.len() >= 3, Error::<T>::TooShortCategories); //check minimum length for the categories
            ensure!(categories.len() <=256, Error::<T>::TooLongCategories); //check maximum length for the categories
            // check categories that must be present
            let mut x=0;
            loop {
                let category=json_get_recordvalue(categories.clone(),x);
				if category.len()==0 {
					break;
				}
                // convert category from vec to u32
				let category_slice=category.as_slice();
            	let category_str=match str::from_utf8(&category_slice){
                	Ok(f) => f,
                	Err(_) => "0"
            	};
            	let categoryvalue:u32 = match u32::from_str(category_str){
                	Ok(f) => f,
                	Err(_) => 0,
            	};
				ensure!(categoryvalue >0, Error::<T>::CategoryCannotBeZero);
                ensure!(ImpactActionsCategories::contains_key(&categoryvalue)==false, Error::<T>::CategoryNotFound);
                x=x+1;
            }
            // check number of auditors required
            let jsa=configuration.clone();
            let auditors=json_get_value(jsa,"auditors".as_bytes().to_vec());
            let auditors_slice=auditors.as_slice();
            let auditors_str=match str::from_utf8(&auditors_slice){
                Ok(f) => f,
                Err(_) => "0"
            };
            let auditorsvalue:u32 = match u32::from_str(auditors_str){
                Ok(f) => f,
                Err(_) => 0,
            };
			ensure!(auditorsvalue > 0, Error::<T>::NumberofAuditorsCannotBeZero); 
            // check startblock
            let jssb=configuration.clone();
            let blockstart=json_get_value(jssb,"blockstart".as_bytes().to_vec());
            let blockstart_slice=blockstart.as_slice();
            let blockstart_str=match str::from_utf8(&blockstart_slice){
                Ok(f) => f,
                Err(_) => "0"
            };
            let blockstartvalue:u32 = match u32::from_str(blockstart_str){
                Ok(f) => f,
                Err(_) => 0,
            };
			ensure!(blockstartvalue > 0, Error::<T>::InvalidBlockStart); //check blockstart that must be > 0
            // check block end
            let jseb=configuration.clone();
            let blockend=json_get_value(jseb,"blockend".as_bytes().to_vec());
            let blockend_slice=blockend.as_slice();
            let blockend_str=match str::from_utf8(&blockend_slice){
                Ok(f) => f,
                Err(_) => "0"
            };
            let blockendvalue:u32 = match u32::from_str(blockend_str){
                Ok(f) => f,
                Err(_) => 0,
            };
			ensure!( blockendvalue> 0, Error::<T>::InvalidBlockEnd); //check blockend  that must be > 0
            // check rewards token
            let jsr=configuration.clone();
            let rewardstoken=json_get_value(jsr,"rewardstoken".as_bytes().to_vec());
            let rewardstoken_slice=rewardstoken.as_slice();
            let rewardstoken_str=match str::from_utf8(&rewardstoken_slice){
                Ok(f) => f,
                Err(_) => "-1"
            };
            let rewardstokenvalue:i32 = match i32::from_str(rewardstoken_str){
                Ok(f) => f,
                Err(_) => -1,
            };
			ensure!(rewardstokenvalue >= 0, Error::<T>::InvalidRewardsToken); //check rewards token that must be >= 0
            // check rewards amount
            let jsam=configuration.clone();
            let rewardsamount=json_get_value(jsam,"rewardsamount".as_bytes().to_vec());
            let rewardsamount_slice=rewardsamount.as_slice();
            let rewardsamount_str=match str::from_utf8(&rewardsamount_slice){
                Ok(f) => f,
                Err(_) => "0"
            };
            let rewardsamountvalue:u32 = match u32::from_str(rewardsamount_str){
                Ok(f) => f,
                Err(_) => 0,
            };
			ensure!(rewardsamountvalue > 0, Error::<T>::InvalidRewardsAmount); //check rewards amount that must be > 0
            // check rewards Oracle
            let jso=configuration.clone();
            let rewardsoracle=json_get_value(jso,"rewardsoracle".as_bytes().to_vec());
            let rewardsoracle_slice=rewardsoracle.as_slice();
            let rewardsoracle_str=match str::from_utf8(&rewardsoracle_slice){
                Ok(f) => f,
                Err(_) => "-1"
            };
            let rewardsoraclevalue:i32 = match i32::from_str(rewardsoracle_str){
                Ok(f) => f,
                Err(_) => -1,
            };
			ensure!(rewardsoraclevalue >= 0, Error::<T>::InvalidRewardsOracle); //check rewards oracle that must be >= 0
            // check rewards Auditors
            let jsau=configuration.clone();
            let rewardsauditors=json_get_value(jsau,"rewardsauditors".as_bytes().to_vec());
            let rewardsauditors_slice=rewardsauditors.as_slice();
            let rewardsauditors_str=match str::from_utf8(&rewardsauditors_slice){
                Ok(f) => f,
                Err(_) => "-1"
            };
            let rewardsauditorsvalue:i32 = match i32::from_str(rewardsauditors_str){
                Ok(f) => f,
                Err(_) => -1,
            };
			ensure!(rewardsauditorsvalue >= 0, Error::<T>::InvalidRewardsAuditors); //check rewards auditors that must be >= 0
            // check Slashing amount for Auditors
            let jsas=configuration.clone();
            let slashingauditors=json_get_value(jsas,"slashingsauditors".as_bytes().to_vec());
            let slashingauditors_slice=slashingauditors.as_slice();
            let slashingauditors_str=match str::from_utf8(&slashingauditors_slice){
                Ok(f) => f,
                Err(_) => "-1"
            };
            let slashingauditorsvalue:i32 = match i32::from_str(slashingauditors_str){
                Ok(f) => f,
                Err(_) => -1,
            };
            ensure!(slashingauditorsvalue >= 0, Error::<T>::InvalidSlashingAuditors); //check slashing amount for auditors that must be >= 0
            // check Max errors for revoking auditor
            let jsme=configuration.clone();
            let maxerrorsauditor=json_get_value(jsme,"maxerrorsauditor".as_bytes().to_vec());
            let maxerrorsauditor_slice=maxerrorsauditor.as_slice();
            let maxerrorsauditor_str=match str::from_utf8(&maxerrorsauditor_slice){
                Ok(f) => f,
                Err(_) => "0"
            };
            let maxerrorsauditorvalue:u32 = match u32::from_str(maxerrorsauditor_str){
                Ok(f) => f,
                Err(_) => 0,
            };
            ensure!(maxerrorsauditorvalue > 0, Error::<T>::InvalidMaxErrorsAuditor); //check max errors for auditors before to be revoked, that must be > 0
            // check custom fields
            let mut x=0;
            let mut vy = Vec::<u8>::new();
            vy.push(b'Y');
            let mut vn = Vec::<u8>::new();
            vn.push(b'N');
            let mut ftn = Vec::<u8>::new();
            ftn.push(b'N');
            let mut fts = Vec::<u8>::new();
            fts.push(b'S');
            loop {
                let jr=json_get_recordvalue(configuration.clone(),x);
				if jr.len()==0 {
					break;
				}
                let fieldname=json_get_value(jr.clone(),"fieldname".as_bytes().to_vec());
                ensure!(fieldname.len() > 0, Error::<T>::FieldNameTooShort); //check minimum length for the fieldname
                let fieldtype=json_get_value(jr.clone(),"fieldtype".as_bytes().to_vec());
                ensure!(fieldtype==fts || fieldtype==ftn, Error::<T>::FieldTypeIsWrong);
                let mandatory=json_get_value(jr.clone(),"mandatory".as_bytes().to_vec());
                ensure!(mandatory==vn || mandatory==vy,Error::<T>::FieldMandatoryFlagIsWrong);
                x=x+1;
                
            }
			// Update deposit
			ImpactActions::insert(uid,configuration.clone());
            // Generate event
			Self::deposit_event(RawEvent::ImpactActionCreated(uid,configuration));
			// Return a successful DispatchResult
			Ok(())
		}
        /// Destroy an impact action
        #[weight = 1000]
		pub fn destroy_impact_action(origin, uid: u32) -> dispatch::DispatchResult {
			// check the request is signed from Super User
			let _sender = ensure_root(origin)?;
            // verify the impact action exists
			ensure!(ImpactActions::contains_key(&uid)==true, Error::<T>::ImpactActionNotFound);
			// Update deposit
			ImpactActions::take(uid);
            // Generate event
            //it can leave orphans, anyway it's a decision of the super user
			Self::deposit_event(RawEvent::ImpactActionDestroyed(uid));
			// Return a successful DispatchResult
			Ok(())
		}
        /// Submit an approval request for an impact action
        #[weight = 10000]
		pub fn request_approval(origin, uid: u32, info: Vec<u8>) -> dispatch::DispatchResult {
			// check the request is signed
			let sender = ensure_signed(origin)?;
			//check info length
			ensure!(info.len()< 4, Error::<T>::TooShortInfo); 
            ensure!(info.len()> 1024, Error::<T>::TooLongInfo); 
			// check the id is > 0
			ensure!(uid > 0, Error::<T>::UidCannotBeZero); 
			// check json validity
			let js=info.clone();
			ensure!(json_check_validity(js),Error::<T>::InvalidJson);
            // check impact action id
            let jsi=info.clone();
            let impactactionid=json_get_value(jsi,"impactactionid".as_bytes().to_vec());
            let impactactionid_slice=impactactionid.as_slice();
            let impactactionid_str=match str::from_utf8(&impactactionid_slice){
                Ok(f) => f,
                Err(_) => "0"
            };
            let impactactionidvalue:u32 = match u32::from_str(impactactionid_str){
                Ok(f) => f,
                Err(_) => 0,
            };
            // check that the impactactionid is present
            ensure!(ImpactActions::contains_key(&impactactionidvalue)==true, Error::<T>::ImpactActionNotFound);
            
            // check that the uid is not already present
			ensure!(ImpactActionsSubmissions::contains_key(&uid)==false, Error::<T>::ImpactActionSubmissionDuplicated);

			// Insert approval request
			ImpactActionsSubmissions::insert(uid,info);
            // Generate event
			Self::deposit_event(RawEvent::ImpactActionRequestApproval(sender,uid));
			// Return a successful DispatchResult
			Ok(())
		}
        
        /// Vote an approval request for an impact action from an auditor or an oracle
        #[weight = 1000]
        pub fn vote_approval_request(origin, approvalid: u32, vote: Vec<u8>) -> dispatch::DispatchResult {
             // check the request is signed 
             let sender = ensure_signed(origin)?;
             //check info length
             ensure!(vote.len()< 4, Error::<T>::TooShortInfo); 
             ensure!(vote.len()> 1024, Error::<T>::TooLongInfo); 
             // check the uid is > 0
             ensure!(approvalid > 0, Error::<T>::UidCannotBeZero); 
             // check json validity
             let js=vote.clone();
             ensure!(json_check_validity(js),Error::<T>::InvalidJson);
             // check vote Y/N
			let jsv=vote.clone();
            let mut vy = Vec::<u8>::new();
            vy.push(b'Y');
            let mut vn = Vec::<u8>::new();
            vn.push(b'N');
			let vote=json_get_value(jsv,"vote".as_bytes().to_vec());
			ensure!(vote==vy || vote==vn, Error::<T>::VoteIsInvalid); 
            // check for otherinfo
            let jso=vote.clone();
            let otherinfo=json_get_value(jso,"otherinfo".as_bytes().to_vec());
            ensure!(otherinfo.len() >2 , Error::<T>::OtherInfoTooShort); //check minimum length for the otherinfo
			ensure!(otherinfo.len() <=1024, Error::<T>::OtherInfoTooLong); //check maximum length for the otherinfo
            // check that the approval id is present
            ensure!(ImpactActionsSubmissions::contains_key(&approvalid)==true, Error::<T>::ImpactActionSubmissionNotFound);
            // check that the auditor is assigned to the approval request
            ensure!(ImpactActionsSubmissionsAuditors::<T>::contains_key(&approvalid,&sender)==true, Error::<T>::SignerNotAssigneAsAuditor);
            // Insert approval request
            ImpactActionsSubmissionsVotes::<T>::insert(approvalid,sender.clone(),vote.clone());
            // Generate event
            Self::deposit_event(RawEvent::ImpactActionRequestApprovalVoted(sender,approvalid,vote));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Create a new category of impact actions
        #[weight = 1000]
		pub fn create_category(origin, uid: u32, description: Vec<u8>) -> dispatch::DispatchResult {
			// check the request is signed from Super User
			let _sender = ensure_root(origin)?;
			//check description length
			ensure!(description.len()< 4, Error::<T>::TooShortDescription); 
            ensure!(description.len()> 128, Error::<T>::TooLongDescription); 
			// check the id is > 0
			ensure!(uid > 0, Error::<T>::UidCannotBeZero); 
			// check that the uid is not already present
			ensure!(ImpactActionsCategories::contains_key(&uid)==false, Error::<T>::DuplicatedCategoryImpactAction);
			// Update categories
			ImpactActionsCategories::insert(uid,description.clone());
            // Generate event 
			Self::deposit_event(RawEvent::ImpactActionCategoryCreated(uid,description));
			// Return a successful DispatchResult
			Ok(())
		}
        /// Destroy a category of impact actions
        #[weight = 1000]
		pub fn destroy_category(origin, uid: u32) -> dispatch::DispatchResult {
			// check the request is signed from Super User
			let _sender = ensure_root(origin)?;
			// check the id is > 0
			ensure!(uid > 0, Error::<T>::UidCannotBeZero); 
			// check that the uid is already present
			ensure!(ImpactActionsCategories::contains_key(&uid)==true, Error::<T>::CategoryImpactActionNotFound);
			// Update Categories
			ImpactActionsCategories::take(uid);
            // Generate event 
			Self::deposit_event(RawEvent::ImpactActionCategoryDestroyed(uid));
			// Return a successful DispatchResult
			Ok(())
		}
        /// Create a new auditor
        #[weight = 1000]
		pub fn create_auditor(origin, account: T::AccountId, configuration: Vec<u8>) -> dispatch::DispatchResult {
			// check the request is signed from Super User
			let _sender = ensure_root(origin)?;
			//check configuration length
			ensure!(configuration.len()< 12, Error::<T>::TooShortConfigurationLength); 
            ensure!(configuration.len()> 8192, Error::<T>::TooLongConfigurationLength); 
			// check that the account is not already present
			ensure!(ImpactActionsAuditors::<T>::contains_key(&account)==false, Error::<T>::DuplicatedImpactActionAuditor);
            // check json validity
			let js=configuration.clone();
			ensure!(json_check_validity(js),Error::<T>::InvalidJson);
            // check description
			let jsd=configuration.clone();
			let description=json_get_value(jsd,"description".as_bytes().to_vec());
			ensure!(description.len() >= 4, Error::<T>::TooShortDescription); //check minimum length for the description
			ensure!(description.len() <=1024, Error::<T>::TooLongDescription); //check maximum length for the description
            // check categories
            let jsc=configuration.clone();
            let categories=json_get_value(jsc,"categories".as_bytes().to_vec());
            ensure!(categories.len() >= 3, Error::<T>::TooShortCategories); //check minimum length for the categories
            ensure!(categories.len() <=256, Error::<T>::TooLongCategories); //check maximum length for the categories
            // check categories that must be present
            let mut x=0;
            loop {
                let category=json_get_recordvalue(categories.clone(),x);
				if category.len()==0 {
					break;
				}
                // convert category from vec to u32
				let category_slice=category.as_slice();
            	let category_str=match str::from_utf8(&category_slice){
                	Ok(f) => f,
                	Err(_) => "0"
            	};
            	let categoryvalue:u32 = match u32::from_str(category_str){
                	Ok(f) => f,
                	Err(_) => 0,
            	};
				ensure!(categoryvalue >0, Error::<T>::CategoryCannotBeZero);
                ensure!(ImpactActionsCategories::contains_key(&categoryvalue)==false, Error::<T>::CategoryNotFound);
                x=x+1;
            }
            let jsd=configuration.clone();
			let area=json_get_value(jsd,"area".as_bytes().to_vec());
			ensure!(area.len() >= 4, Error::<T>::TooShortArea); //check minimum length for the area
			ensure!(area.len() <=128, Error::<T>::TooLongArea); //check maximum length for the area
            // check otherinfo
			let jso=configuration.clone();
			let otherinfo=json_get_value(jso,"otherinfo".as_bytes().to_vec());
			ensure!(otherinfo.len() <=1024, Error::<T>::TooLongOtherInfo); //check maximum length for the other info
            // check minimum stakes required
            let jsms=configuration.clone();
            let stakesmin=json_get_value(jsms,"stakesmin".as_bytes().to_vec());
            let stakesmin_slice=stakesmin.as_slice();
            let stakesmin_str=match str::from_utf8(&stakesmin_slice){
                Ok(f) => f,
                Err(_) => "-1"
            };
            let stakesminvalue:i32 = match i32::from_str(stakesmin_str){
                Ok(f) => f,
                Err(_) => -1,
            };
			ensure!(stakesminvalue >= 0, Error::<T>::InvalidStakesMinimum); //check stakes that must be >= 0
			// insert new auditor
			ImpactActionsAuditors::<T>::insert(account.clone(),configuration.clone());
            // Generate event 
			Self::deposit_event(RawEvent::ImpactActionAuditorCreated(account,configuration));
			// Return a successful DispatchResult
			Ok(())
		}
        /// Destroy an auditor
        #[weight = 1000]
		pub fn destroy_auditor(origin, account: T::AccountId) -> dispatch::DispatchResult {
			// check the request is signed from Super User
			let _sender = ensure_root(origin)?;
			// check that the uid is already present
			ensure!(ImpactActionsAuditors::<T>::contains_key(&account)==true, Error::<T>::AuditorImpactActionNotFound);
			// Update Auditor
			ImpactActionsAuditors::<T>::take(account.clone());
            // Generate event 
			Self::deposit_event(RawEvent::ImpactActionAuditorDestroyed(account));
			// Return a successful DispatchResult
			Ok(())
		}
        /// Assign an auditor
        #[weight = 1000]
		pub fn assign_auditor(origin, uid: u32, auditor: T::AccountId, maxdays: u32) -> dispatch::DispatchResult {
            // get the proxy account used for assigning the auditor
            let proxy=ImpactActionsProxy::<T>::get(0).unwrap();
			// check the request is signed from Super User
			let sender = ensure_signed(origin)?;
            ensure!(sender==proxy,Error::<T>::SigningAccountNotValidProxy);
			// check the uid is > 0
			ensure!(uid > 0, Error::<T>::UidCannotBeZero); 
			// check that the uid is already present
			ensure!(ImpactActionsSubmissions::contains_key(&uid)==true, Error::<T>::ImpactActionSubmissionNotFound);
			// check that the auditor is already present
			ensure!(ImpactActionsAuditors::<T>::contains_key(&auditor)==true, Error::<T>::AuditorImpactActionNotFound);
            // check the max days >0
            ensure!(maxdays > 0, Error::<T>::MaxDaysCannotBeZero); 
			// Update Assigned Auditors
			ImpactActionsSubmissionsAuditors::<T>::insert(uid,auditor.clone(),maxdays);
            // Generate event 
			Self::deposit_event(RawEvent::ImpactActionAuditorAssigned(uid,auditor,maxdays));
			// Return a successful DispatchResult
			Ok(())
		}
        /// Create a new oracle
        #[weight = 1000]
		pub fn create_oracle(origin, uid: u32, configuration: Vec<u8>) -> dispatch::DispatchResult {
			// check the request is signed from Super User
			let _sender = ensure_root(origin)?;
			//check configuration length
			ensure!(configuration.len()< 4, Error::<T>::TooShortConfigurationLength); 
            ensure!(configuration.len()> 1024, Error::<T>::TooLongConfigurationLength); 
			// check the id is > 0
			ensure!(uid > 0, Error::<T>::UidCannotBeZero); 
			// check that the uid is not already present
			ensure!(ImpactActionsOracles::contains_key(&uid)==false, Error::<T>::DuplicatedImpactAction);
            // check json validity
			let js=configuration.clone();
			ensure!(json_check_validity(js),Error::<T>::InvalidJson);
            // check description
			let jsd=configuration.clone();
			let description=json_get_value(jsd,"description".as_bytes().to_vec());
			ensure!(description.len() >= 4, Error::<T>::TooShortDescription); //check minimum length for the description
			ensure!(description.len() <=1024, Error::<T>::TooLongDescription); //check maximum length for the description
            // check accountid in base58
            let jsc=configuration.clone();
            let oracleaccount=json_get_value(jsc,"account".as_bytes().to_vec());
            ensure!(oracleaccount.len() == 48, Error::<T>::OracleAccountNotValid); //check length for the account
            // check other info field as mandatory
            let jso=configuration.clone();
            let otherinfo=json_get_value(jso,"otherinfo".as_bytes().to_vec());
            ensure!(otherinfo.len() > 0, Error::<T>::OtherInfoMissing); 
			ImpactActionsOracles::insert(uid,configuration.clone());
            // Generate event
			Self::deposit_event(RawEvent::ImpactActionOracleCreated(uid,configuration));
			// Return a successful DispatchResult
			Ok(())
		}
        /// Destroy an oracle
        #[weight = 1000]
		pub fn destroy_oracle(origin, uid: u32) -> dispatch::DispatchResult {
			// check the request is signed from Super User
			let _sender = ensure_root(origin)?;
			// check the id is > 0
			ensure!(uid > 0, Error::<T>::UidCannotBeZero); 
			// check that the uid is already present
			ensure!(ImpactActionsOracles::contains_key(&uid)==true, Error::<T>::OracleImpactActionNotFound);
			// Update Categories
			ImpactActionsOracles::take(uid);
            // Generate event 
			Self::deposit_event(RawEvent::ImpactActionOracleDestroyed(uid));
			// Return a successful DispatchResult
			Ok(())
		}
        /// Create a new proxy account (uid=0 for Assigning Auditors)
        #[weight = 1000]
		pub fn create_proxy(origin, uid: u32, proxy: T::AccountId) -> dispatch::DispatchResult {
			// check the request is signed from Super User
			let _sender = ensure_root(origin)?;
			// check that the uid is not already present
			ensure!(ImpactActionsProxy::<T>::contains_key(&uid)==false, Error::<T>::DuplicatedProxyId);

            // insert the proxy account
			ImpactActionsProxy::<T>::insert(uid,proxy);
            // Generate event 
			Self::deposit_event(RawEvent::ImpactActionProxyCreated(uid));
			// Return a successful DispatchResult
			Ok(())
		}
        /// Destroy a proxy account
        #[weight = 1000]
		pub fn destroy_proxy(origin, uid: u32) -> dispatch::DispatchResult {
			// check the request is signed from Super User
			let _sender = ensure_root(origin)?;
			// check the id is > 0
			ensure!(uid > 0, Error::<T>::UidCannotBeZero); 
			// check that the uid is already present
			ensure!(ImpactActionsProxy::<T>::contains_key(&uid)==true, Error::<T>::ProxyAccountNotFound);
			// Update Categories
			ImpactActionsProxy::<T>::take(uid);
            // Generate event 
			Self::deposit_event(RawEvent::ImpactActionProxyDestroyed(uid));
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




