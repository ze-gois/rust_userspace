use crate::file::format::elf::header::{Header64, Identifier};
use crate::file::traits::Readable;

use super::constants::{ELFCLASS64, ELFDATA2LSB, EM_X86_64, ET_DYN, ET_EXEC, EV_CURRENT};
use super::error::Error;
use super::io::read_at;

pub(super) fn validate_header(header: Header64, endianness: bool) -> Result<(), Error> {
    if header.e_ident.class.0 != ELFCLASS64 {
        return Err(Error::UnsupportedClass);
    }
    if header.e_ident.data.0 != ELFDATA2LSB || !endianness {
        return Err(Error::UnsupportedEndianness);
    }
    if header.e_ident.version.0 != EV_CURRENT {
        return Err(Error::InvalidHeader);
    }
    if header.e_type.0 != ET_EXEC && header.e_type.0 != ET_DYN {
        return Err(Error::UnsupportedType);
    }
    if header.e_machine.0 != EM_X86_64 {
        return Err(Error::UnsupportedMachine);
    }
    if header.e_ehsize.0 as usize
        != <Header64 as ample::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE
    {
        return Err(Error::InvalidHeader);
    }
    Ok(())
}

pub(super) fn read_header(file_descriptor: isize) -> Result<(Header64, bool), Error> {
    let identifier_bytes = read_at::<
        { <Identifier as ample::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE },
    >(file_descriptor, 0)?;
    let identifier = Identifier::read_from_pointer(identifier_bytes.as_ptr(), 0, true).0;
    if !identifier.is_magical() {
        return Err(Error::InvalidHeader);
    }
    let endianness = match identifier.data() {
        crate::file::format::elf::header::identifier::Data::DataLSB => true,
        _ => return Err(Error::UnsupportedEndianness),
    };
    let header_bytes = read_at::<
        { <Header64 as ample::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE },
    >(file_descriptor, 0)?;
    let header = Header64::read_from_pointer(header_bytes.as_ptr(), 0, endianness).0;
    Ok((header, endianness))
}
