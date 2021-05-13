use crate::errors::{MUST_ASYNC, MUST_NOT_ASYNC, UNSUPPORTED_GENERICS, UNSUPPORTED_RECEIVER};
use crate::ClientKind;
use syn::{Error, FnArg, Receiver, Result, Signature, TraitItemMethod};

pub(crate) fn check_async(method: &TraitItemMethod, kind: &ClientKind) -> Result<()> {
    let asyncness = &method.sig.asyncness;
    match kind {
        ClientKind::Futures | ClientKind::FuturesNoSend => {
            if asyncness.is_some() {
                Ok(())
            } else {
                Err(Error::new_spanned(&method.sig, MUST_ASYNC))
            }
        }
        ClientKind::Blocking => {
            if let Some(asyncness) = asyncness {
                Err(Error::new_spanned(&asyncness, MUST_NOT_ASYNC))
            } else {
                Ok(())
            }
        }
    }
}

pub(crate) fn check_no_generics(method: &TraitItemMethod) -> Result<()> {
    let sig = &method.sig;
    check_no_generic_params(sig)?;
    check_no_where_clause(sig)?;
    Ok(())
}
fn check_no_generic_params(sig: &Signature) -> Result<()> {
    if sig.generics.params.is_empty() {
        Ok(())
    } else {
        Err(Error::new_spanned(&sig.generics, UNSUPPORTED_GENERICS))
    }
}

fn check_no_where_clause(sig: &Signature) -> Result<()> {
    if let Some(where_clause) = sig.generics.where_clause.as_ref() {
        Err(Error::new_spanned(where_clause, UNSUPPORTED_GENERICS))
    } else {
        Ok(())
    }
}

pub(crate) fn check_correct_receiver(method: &TraitItemMethod) -> Result<()> {
    let receiver = get_receiver(method)
        .ok_or_else(|| Error::new_spanned(&method.sig, UNSUPPORTED_RECEIVER))?;
    get_good_mutability(receiver).ok_or_else(|| Error::new_spanned(receiver, UNSUPPORTED_RECEIVER))
}

fn get_receiver(method: &TraitItemMethod) -> Option<&Receiver> {
    let sig = &method.sig;
    let first_arg = sig.inputs.first()?;

    match first_arg {
        FnArg::Receiver(receiver) => Some(receiver),
        _ => None,
    }
}

fn get_good_mutability(receiver: &Receiver) -> Option<()> {
    let (_, lifetime) = receiver.reference.as_ref()?;
    if lifetime.is_none() && receiver.mutability.is_none() {
        Some(())
    } else {
        None
    }
}
