


#[macro_export]
macro_rules! try_opt { 
  ($e: expr) => { 
    match $e { 
      Some(x) => x, 
      None => return None 
      } 
  } 
}

#[macro_export]
macro_rules! try_opt_resbox { 

  ($e: expr, $s: expr) => { 
    match $e { 
      Some(x) => x, 
      None => return Err(stringerror::stringBoxErr($s)),
      // None => return Err(Box::new(stringerror::Error::new($s))), 
      } 
  } 
}

/*
#[macro_export]
macro_rules! try_opt_res {
  ($e: expr, $s: expr) =>  
    (match $e {
      Some(val) => val,
      None => {
          let err = Error::new($s) ;
          return Err(std::convert::From::from(err))
      },
    })
}
*/
/*
hmmm not getting why this doesn't work... want it to be like the try! macro.

macro_rules! try_opt_res {
  ($e: expr, $s: expr) =>  
    (match $e {
      Some(val) => val,
      None => {
          let err = Err(Error::new(ErrorKind::Other, $s)) ;
          return $crate::result::Result::Err($crate::convert::From::from(err))
      },
    })
}
*/


