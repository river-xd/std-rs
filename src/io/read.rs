
use macros::method;
use crate::UnwrapExt;
use wasm_bindgen::prelude::wasm_bindgen;

use wasm_bindgen_futures::{
  JsFuture,
  future_to_promise as promise
};

use tokio::io::{
  ReadBuf,
  AsyncRead,
  AsyncReadExt
};

use js_sys::{
  Promise,
  Function,
  Uint8Array,
  wasm_bindgen::{
    JsValue,
    UnwrapThrowExt
  }
};


use std::{
  mem,
  pin::Pin,
  future::Future,
  io::{
    self,
    Read
  },
  task::{
    Poll,
    Context
  }
};



pub struct AsyncReader {
  this: JsValue,
  read: Function
}

impl AsyncRead for AsyncReader {
  fn poll_read(self: Pin<&mut Self>,cx: &mut Context<'_>,buf: &mut ReadBuf<'_>,)-> Poll<io::Result<()>> {
    let buf=unsafe { Uint8Array::view(std::mem::transmute(buf.inner_mut())) };
    let res=self.read.call1(&self.this,&buf).unwrap_or_throw();
    let mut future=JsFuture::from(Promise::from(res));
    let poll=JsFuture::poll(Pin::new(&mut future),cx);

    poll.map(|res| {
      res.unwrap_or_throw();
      Ok(())
    })
  }
}

impl AsyncReader {
  #[inline]
  fn new(this: JsValue,read: Function)-> Self {
    Self {
      read,
      this
    }
  }
}



pub struct Reader {
  this: JsValue,
  read: Function
}

impl Reader {
  #[inline]
  fn new(this: JsValue,read: Function)-> Self {
    Self {
      read,
      this
    }
  }
}

impl Read for Reader {
  #[inline]
  fn read(&mut self,buf: &mut [u8])-> io::Result<usize> {
    let buf=unsafe { Uint8Array::view(buf) };
    Ok(
      self.read.call1(&self.this,&buf)
      .unwrap_or_throw()
      .unchecked_into_f64() as _
    )
  }
}


macro_rules! handle_future {
  ($promise:expr)=> {
    match $promise.await {
      Ok(res)=> Ok(JsValue::from(res as u32)),
      Err(err)=> Err(mem::transmute::<_,u8>(err.kind()).into())
    }
  }
}


#[method]
pub unsafe fn read_to_end(buf: &mut Vec<u8>,this: JsValue,read: Function)-> Promise {
  promise(async {
    handle_future!(AsyncReader::new(this,read).read_to_end(buf))
  })
}

<<<<<<< HEAD
#[wasm_bindgen]
<<<<<<< HEAD
pub fn read_exact_sync(reader: JsValue,read: Function,buf: &mut [u8]) {
  Reader::new(reader,read)
  .read_exact(buf)
  .unwrap_throw()
=======
pub unsafe fn read_exact(this: JsValue,read: Function,ptr: *mut u8,len: usize)-> Promise {
  promise(async move {
    let buf=std::slice::from_raw_parts_mut(ptr,len);
    handle_future!(AsyncReader::new(this,read).read_exact(buf))
  })
>>>>>>> 82fa756 (improved code (read low-level-impl))
}

=======
>>>>>>> 4cc9e6b (implemented low-level read_exact_sync)


#[method]
pub fn read_to_end_sync(buf: &mut Vec<u8>,this: JsValue,read: Function)-> usize {
  Reader::new(this,read)
  .read_to_end(buf)
  .unwrap_throw()
}

#[wasm_bindgen]
pub fn read_exact_sync(this: JsValue,read: Function,buf: &mut [u8]) {
  Reader::new(this,read)
  .read_exact(buf)
  .unwrap_throw()
}

