use std::{convert::TryInto, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disk {
    pub path: PathBuf,
    pub extended: bool,
    pub creator: String,
    pub num_tracks: u8,
    pub num_sides: u8,
    pub track_size: u16,
    pub tracks: Vec<Track>,
}

impl Disk {
    pub fn load(rom: Vec<u8>, path: PathBuf) -> std::io::Result<Disk> {
        // TODO: use custom Result instead of std::io:Result
        log::debug!("Loading DSK file.");

        let contents = rom;

        let standard_header = b"MV - CPCEMU Disk-File\r\nDisk-Info\r\n";
        let extended_header = b"EXTENDED CPC DSK File\r\nDisk-Info\r\n";
        let extended = if contents[0..0x08] == standard_header[0..0x8] {
            // some disk images have a slightly different header. so don't compare the whole header
            false
        } else if contents[0..0x22] == extended_header[0..0x22] {
            true
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Could not find the expected file header.",
            ));
        };

        let creator = String::from_utf8(contents[0x22..0x30].to_vec()).unwrap_or_default();

        let num_tracks = contents[0x30];
        let num_sides = contents[0x31];
        let track_size = u16::from_le_bytes(contents[0x32..0x34].try_into().unwrap());

        let header = b"Track-Info\r\n";
        let mut tracks = Vec::new();
        for track in 0..num_tracks {
            for side in 0..num_sides {
                let track_start = if extended {
                    let mut offset_sum = 0x100usize;
                    for i in 0..(num_sides as usize * track as usize + side as usize) {
                        offset_sum += (contents[0x34 + i] as usize) << 8;
                    }
                    offset_sum
                } else {
                    track_size as usize * (num_sides as usize * track as usize + side as usize)
                        + 0x100
                };
                match contents[track_start..(track_start + 0x0c)].cmp(header) {
                    std::cmp::Ordering::Equal => {
                        let track = contents[track_start + 0x10]; // TODO: verify this is the same as the shadowed value?
                        let side = contents[track_start + 0x11]; // TODO: verify this is the same as the shadowed value?
                        let sector_size = contents[track_start + 0x14];
                        let num_sectors = contents[track_start + 0x15];
                        let gap3_length = contents[track_start + 0x16];
                        let filler_byte = contents[track_start + 0x15];

                        let mut sector_infos = Vec::new();
                        let mut sectors = Vec::new();
                        for sector in 0..num_sectors {
                            let sector_info_start = 8 * sector as usize + track_start + 0x18;
                            let sector_data_start = (0x80 << sector_size) as usize
                                * sector as usize
                                + track_start
                                + 0x100;

                            log::trace!(
                                "track {}, sector {} (id = {})",
                                contents[sector_info_start],
                                sector,
                                contents[sector_info_start + 0x02]
                            );

                            let actual_length = if extended {
                                Some(u16::from_le_bytes(contents[0x32..0x34].try_into().unwrap()))
                            } else {
                                None
                            };

                            sector_infos.push(SectorInfo {
                                track: contents[sector_info_start], // TODO: verify this is the same as above?
                                side: contents[sector_info_start + 0x01], // TODO: verify this is the same as above?
                                sector_id: contents[sector_info_start + 0x02],
                                sector_size: contents[sector_info_start + 0x03], // TODO: verify this is the same as above?
                                fdc_status1: contents[sector_info_start + 0x04],
                                fdc_status2: contents[sector_info_start + 0x05],
                                actual_length,
                            });

                            sectors.push(
                                contents[sector_data_start
                                    ..(sector_data_start + (0x80 << sector_size) as usize)]
                                    .to_vec(),
                            );
                        }

                        tracks.push(Track {
                            track,
                            side,
                            sector_size,
                            num_sectors,
                            gap3_length,
                            filler_byte,
                            sector_infos,
                            sectors,
                        });
                    }
                    _ => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Could not find the expected track header.",
                        ))
                    }
                }
            }
        }

        let disk = Disk {
            path,
            extended,
            creator,
            num_tracks,
            num_sides,
            track_size,
            tracks,
        };

        Ok(disk)
    }

    pub fn find_track_index(&self, track: u8, side: u8) -> Option<usize> {
        // TODO: handle out of bounds errors
        Some((track * self.num_sides + side) as usize)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub track: u8,
    pub side: u8,
    pub sector_size: u8,
    pub num_sectors: u8,
    pub gap3_length: u8,
    pub filler_byte: u8,
    pub sector_infos: Vec<SectorInfo>,
    pub sectors: Vec<Vec<u8>>,
}

impl Track {
    pub fn find_sector(&self, sector_id: u8) -> Option<usize> {
        self.sector_infos
            .iter()
            .position(|sector_info| sector_info.sector_id == sector_id)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectorInfo {
    pub track: u8,
    pub side: u8,
    pub sector_id: u8,
    pub sector_size: u8,
    pub fdc_status1: u8, // TODO: do we actually use this?
    pub fdc_status2: u8, // TODO: do we actually use this?
    pub actual_length: Option<u16>,
}
