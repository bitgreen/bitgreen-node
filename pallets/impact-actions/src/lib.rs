#![cfg_attr(not(feature = "std"), no_std)]

/// Module to manage the impact actions on BitGreen Blockchain

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure, weights::Pays, traits::Currency};
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
	trait Store for Module<T: Config> as bitgclaim {
		// we use a safe crypto hashing with blake2_128
		// We store the impact actions configuration 
		ImpactActions get(fn get_impactaction): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        // Categories for impact actions
        ImpactActionsCategories get(fn get_categories_impactactions): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
        // Auditor configuration
        ImpactActionsAuditors get(fn get_auditors_impactaction): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
	}
}



// We generate events to inform the users of succesfully actions.
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
	    ImpactActionCreated(AccountId,u32),
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
        /// Too short auditors for the impact action, it must be >=2.
        TooShortAuditors,
        /// Too short auditors for the impact action, it must be <=256.
        TooLongAuditors,
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
        // Minimum Stakes must be >=0
        InvalidStakesMinimum,
        // Other info is too long, it must be < 1024
        TooLongOtherInfo,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized
		type Error = Error<T>;
		// Events must be initialized
		fn deposit_event() = default;
		/// Create a new impact action
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
            // TODO - Add control on state of categories received that must be present
            // check categories
            let jsa=configuration.clone();
            let auditors=json_get_value(jsa,"auditors".as_bytes().to_vec());
            ensure!(auditors.len() >= 2, Error::<T>::TooShortAuditors); //check minimum length for the auditors (can be empty with [])
            ensure!(auditors.len() <=256, Error::<T>::TooLongAuditors); //check maximum length for the auditors
            // TODO - Add control on state of auditors received that must be present
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

			// Update deposit
			ImpactActions::insert(uid,configuration);
            // Generate event
			//Self::deposit_event(RawEvent::ImpactActionCreated(sender,uid));
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
            //TODO: verify it's not leaving orphans, in case deny
			//Self::deposit_event(RawEvent::ImpactActionCreated(sender,uid));
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
			ImpactActionsCategories::insert(uid,description);
            // Generate event (TODO)
			//Self::deposit_event(RawEvent::ImpactActionCreated(sender,uid));
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
            // Generate event (TODO)
			//Self::deposit_event(RawEvent::ImpactActionCreated(sender,uid));
			// Return a successful DispatchResult
			Ok(())
		}
        /// Create a new impact action
        #[weight = 1000]
		pub fn create_auditor(origin, uid: u32, configuration: Vec<u8>) -> dispatch::DispatchResult {
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
            // TODO - Add control on state of categories received that must be present
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
            
			// Update deposit
			ImpactActionsAuditors::insert(uid,configuration);
            // Generate event (TODO)
			//Self::deposit_event(RawEvent::ImpactActionCreated(sender,uid));
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




