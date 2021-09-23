#![cfg_attr(not(feature = "std"), no_std)]

/// Module to manage the Bonds on BitGreen Blockchain

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure, traits::Currency};
use frame_system::{ensure_root,ensure_signed};
use sp_std::prelude::*;
use core::str;
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
        //
		// Settings configuration, we store json structure with different keys (see the function for details)
		Settings get(fn get_settings): map hasher(blake2_128_concat) Vec<u8> => Option<Vec<u8>>;
        Kyc get(fn get_kyc): map hasher(blake2_128_concat) u32 => Option<Vec<u8>>;
	}
}

// We generate events to inform the users of succesfully actions.
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
	    SettingsCreated(Vec<u8>,Vec<u8>),               // New settings configuration has been created
        SettingsDestroyed(Vec<u8>),                     // A settings has been removed
        BondIssued(AccountId,Vec<u8>),                  // Placeholder for account id, to be removed...
        KycStored(u32,Vec<u8>),                         // Kyc data stored on chain
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
        /// key=="keyc" {"manager":"xxxaccountidxxx","supervisor":"xxxxaccountidxxxx","operators":["xxxxaccountidxxxx","xxxxaccountidxxxx",...]}
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
        pub fn create_change_kyc(origin, id: u32, info: Vec<u8>) -> dispatch::DispatchResult {
            let signer = ensure_signed(origin)?;
            // TODO check the signer is one of the operators for kyc
            //check id >0
            ensure!(id>0, Error::<T>::KycIdIsWrongCannotBeZero); 
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
            if Kyc::contains_key(&id)==false {
                // Insert kyc
                Kyc::insert(id.clone(),info.clone());
            } else {
                // Replace Kyc Data 
                Kyc::take(id.clone());
                Kyc::insert(id.clone(),info.clone());
            }
            // Generate event
            Self::deposit_event(RawEvent::KycStored(id,info));
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



