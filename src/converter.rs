use Converter;
use ConverterFlags;
use ConverterResult;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::mem;
use std::ptr;

pub trait ConverterExtManual {
    fn convert<IN: AsRef<[u8]>, OUT: AsMut<[u8]>>(
        &self,
        inbuf: IN,
        outbuf: OUT,
        flags: ConverterFlags)
        -> Result<(ConverterResult, usize, usize), Error>;
}

impl<O: IsA<Converter>> ConverterExtManual for O {
    fn convert<IN: AsRef<[u8]>, OUT: AsMut<[u8]>>(
        &self,
        inbuf: IN,
        outbuf: OUT,
        flags: ConverterFlags)
        -> Result<(ConverterResult, usize, usize), Error> {
        let inbuf: Box<IN> = Box::new(inbuf);
        let (inbuf_size, inbuf) = {
            let slice = (*inbuf).as_ref();
            (slice.len(), slice.as_ptr())
        };
        let mut outbuf: Box<OUT> = Box::new(outbuf);
        let (outbuf_size, outbuf) = {
            let slice = (*outbuf).as_mut();
            (slice.len(), slice.as_mut_ptr())
        };
        unsafe {
            let mut bytes_read = mem::uninitialized();
            let mut bytes_written = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = ffi::g_converter_convert(self.to_glib_none().0,
                                               mut_override(inbuf),
                                               inbuf_size,
                                               outbuf,
                                               outbuf_size,
                                               flags.to_glib(),
                                               &mut bytes_read,
                                               &mut bytes_written,
                                               &mut error);
            if error.is_null() {
                Ok((from_glib(ret), bytes_read, bytes_written))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}