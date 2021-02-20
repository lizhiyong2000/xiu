use super::errors::ControlMessagesError;


use crate::messages::define::msg_type;
use byteorder::{BigEndian, LittleEndian};
use liverust_lib::netio::bytes_writer::AsyncBytesWriter;
use tokio::prelude::*;

pub struct ControlMessages<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    writer: AsyncBytesWriter<S>,
    //amf0_writer: Amf0Writer,
}

impl<S> ControlMessages<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(writer: AsyncBytesWriter<S>) -> Self {
        Self { writer: writer }
    }
    pub fn write_control_message_header(
        &mut self,
        msg_type_id: u8,
        len: u32,
    ) -> Result<(), ControlMessagesError> {
        //0 1 2 3 4 5 6 7
        //+-+-+-+-+-+-+-+-+
        //|fmt|  cs id  |
        //+-+-+-+-+-+-+-+-+
        // 0x0     0x02
        self.writer.write_u8(0x0 << 6 | 0x02)?; //fmt 0 and csid 2
        self.writer.write_u24::<BigEndian>(0)?; //timestamp 3 bytes and value 0
        self.writer.write_u32::<BigEndian>(len)?; //msg length
        self.writer.write_u8(msg_type_id)?; //msg type id
        self.writer.write_u32::<BigEndian>(0)?; //msg stream ID 0

        Ok(())
    }
    pub fn write_set_chunk_size(&mut self, chunk_size: u32) -> Result<(), ControlMessagesError> {
        self.write_control_message_header(msg_type::SET_CHUNK_SIZE, 4)?;
        self.writer
            .write_u32::<BigEndian>(chunk_size & 0x7FFFFFFF)?; //first bit must be 0

        self.writer.flush();
        Ok(())
    }

    pub fn write_abort_message(
        &mut self,
        chunk_stream_id: u32,
    ) -> Result<(), ControlMessagesError> {
        self.write_control_message_header(msg_type::ABORT, 4)?;
        self.writer.write_u32::<BigEndian>(chunk_stream_id)?;

        self.writer.flush();
        Ok(())
    }

    pub fn write_acknowledgement(
        &mut self,
        sequence_number: u32,
    ) -> Result<(), ControlMessagesError> {
        self.write_control_message_header(msg_type::ACKNOWLEDGEMENT, 4)?;
        self.writer.write_u32::<BigEndian>(sequence_number)?;

        self.writer.flush();
        Ok(())
    }

    pub fn write_window_acknowledgement_size(
        &mut self,
        window_size: u32,
    ) -> Result<(), ControlMessagesError> {
        self.write_control_message_header(msg_type::WIN_ACKNOWLEDGEMENT_SIZE, 4)?;
        self.writer.write_u32::<BigEndian>(window_size)?;

        self.writer.flush();
        Ok(())
    }

    pub fn write_set_peer_bandwidth(
        &mut self,
        window_size: u32,
        limit_type: u8,
    ) -> Result<(), ControlMessagesError> {
        self.write_control_message_header(msg_type::SET_PEER_BANDWIDTH, 4)?;
        self.writer.write_u32::<BigEndian>(window_size)?;
        self.writer.write_u8(limit_type)?;

        self.writer.flush();

        Ok(())
    }
}
