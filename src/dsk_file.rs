use std::convert::TryInto;
use std::io::Read;


pub struct Disk {
    extended: bool,
    creator: String,
    num_tracks: u8,
    num_sides: u8,
    track_size: u16,
    tracks: Vec<Track>,
}

impl Disk {
    pub fn load(filename: &str) -> std::io::Result<Disk> {
        let mut file = std::fs::File::open(filename)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let header = b"MV - CPCEMU Disk-File\r\nDisk-Info\r\n";
        let extended = match contents[0..0x22].cmp(header) {
            std::cmp::Ordering::Equal => false,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Could not find the expected file header.")),
        };
        
        let creator = match String::from_utf8(contents[0x22..0x30].to_vec()) {
            Ok(creator) => creator,
            _ => String::new(),
        };

        let num_tracks = contents[0x30];
        let num_sides = contents[0x31];
        let track_size = u16::from_le_bytes(contents[0x32..0x34].try_into().unwrap());

        let header = b"Track-Info\r\n";
        let mut tracks = Vec::new();
        for track in 0..num_tracks {
            for side in 0..num_sides {
                let track_start = track_size as usize * (num_sides as usize * track as usize + side as usize) + 0x100;
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
                            let sector_data_start = sector_size as usize * sector as usize + track_start + 0x100;

                            sector_infos.push(SectorInfo {
                                track: contents[sector_info_start + 0x00], // TODO: verify this is the same as above?
                                side: contents[sector_info_start + 0x01], // TODO: verify this is the same as above?
                                sector_id: contents[sector_info_start + 0x02],
                                sector_size: contents[sector_info_start + 0x03], // TODO: verify this is the same as above?
                                fdc_status1: contents[sector_info_start + 0x04],
                                fdc_status2: contents[sector_info_start + 0x05],
                            });

                            sectors.push(contents[sector_data_start..(sector_data_start + sector_size as usize)].to_vec());
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
                    _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Could not find the expected track header.")),
                }
            }
        }

        let disk = Disk {
            extended,
            creator,
            num_tracks,
            num_sides,
            track_size,
            tracks,
        };

        Ok(disk)
    }
}

struct Track {
    track: u8,
    side: u8,
    sector_size: u8,
    num_sectors: u8,
    gap3_length: u8,
    filler_byte: u8,
    sector_infos: Vec<SectorInfo>,
    sectors: Vec<Vec<u8>>,
}

struct SectorInfo {
    track: u8,
    side: u8,
    sector_id: u8,
    sector_size: u8,
    fdc_status1: u8,
    fdc_status2: u8,
}
