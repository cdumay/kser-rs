use cdumay_result::ExecResult;
use cdumay_error::{Error, ErrorBuilder};
use crate::errors::KserErrors;
use crate::messages::MessageProperties;
use crate::Result;


pub trait Task<M: MessageProperties> {
    
    fn required_fields() -> Option<Vec<String>> { None }
    fn label(msg: &M, action: Option<&str>) -> String {
        format!(
            "{}[{}]{}", msg.entrypoint(), msg.uuid(), match action {
                Some(data) => format!(" - {}", data),
                None => String::new()
            }
        )
    }

    fn check_required_params(msg: &M) -> Result {
        match Self::required_fields() {
            None => Ok(()),
            Some(required_fields) => {
                for attr in required_fields {
                    if msg.params().get(&attr) == None {
                        return Err(
                            ErrorBuilder::from(KserErrors::ValidationError)
                                .message(format!("required field '{}' not set!", &attr))
                                .build()
                        );
                    }
                }
                Ok(())
            }
        }
    }


    fn _post_init(msg: &mut M) -> Result {
        Self::check_required_params(msg)?;
        Self::post_init(msg)
    }
    fn _on_error(msg: &mut M) -> Result {
        error!("{}: {:?}", Self::label(msg, Some("Failed")), msg.result());
        Self::on_error(msg)
    }
    fn _on_success(msg: &mut M) -> Result {
        info!("{}: {:?}", Self::label(msg, Some("Success")), msg.result());
        Self::on_success(msg)
    }
    fn _pre_run(msg: &mut M) -> Result {
        debug!("{}", Self::label(msg, Some("PreRun")));
        Self::pre_run(msg)
    }
    fn _post_run(msg: &mut M) -> Result {
        debug!("{}: {:?}", Self::label(msg, Some("PostRun")), msg.result());
        Self::post_run(msg)
    }
    fn _run(msg: &mut M) -> Result {
        debug!("{}: {:?}", Self::label(msg, Some("Run")), msg.result());
        Self::run(msg)
    }
    fn unsafe_execute(msg: &mut M) -> Result {
        Self::_post_init(msg)?;
        Self::_pre_run(msg)?;
        Self::_run(msg)?;
        Self::_post_run(msg)?;
        Self::_on_success(msg)
    }
    fn execute(msg: &mut M) -> Result {
        match Self::unsafe_execute(msg) {
            Ok(data) => Ok(data),
            Err(err) => {
                let res = ExecResult::from(err);
                *msg.result_mut() = msg.result().clone() + res;
                Self::_on_error(msg)
            }
        }
    }

    fn post_init(msg: &M) -> Result {
        Ok(())
    }
    fn on_error(msg: &mut M) -> Result {
        Ok(())
    }
    fn on_success(msg: &mut M) -> Result {
        Ok(())
    }
    fn pre_run(msg: &M) -> Result {
        Ok(())
    }
    fn post_run(msg: &M) -> Result {
        Ok(())
    }
    fn run(msg: &mut M) -> Result {
        Err(Error::from(KserErrors::NotImplemented))
    }
}

