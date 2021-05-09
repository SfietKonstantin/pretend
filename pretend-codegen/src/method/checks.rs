use crate::errors::{IError, IResult};
use syn::{FnArg, Receiver, TraitItemMethod};

pub(crate) fn check_no_generics(method: &TraitItemMethod) -> IResult<()> {
    let sig = &method.sig;
    if sig.generics.params.is_empty() && sig.generics.where_clause.is_none() {
        Ok(())
    } else {
        Err(IError::UnsupportedGenerics(sig.ident.to_string()))
    }
}

pub(crate) fn check_correct_receiver(method: &TraitItemMethod) -> IResult<()> {
    let make_err = || IError::UnsupportedReceiver(method.sig.ident.to_string());
    let receiver = get_receiver(method).ok_or_else(make_err)?;

    let (_, lifetime) = receiver.reference.as_ref().ok_or_else(make_err)?;
    if lifetime.is_none() && receiver.mutability.is_none() {
        Ok(())
    } else {
        Err(make_err())
    }
}

fn get_receiver(method: &TraitItemMethod) -> Option<&Receiver> {
    let sig = &method.sig;
    let first_arg = sig.inputs.first()?;

    match first_arg {
        FnArg::Receiver(receiver) => Some(receiver),
        _ => None,
    }
}
