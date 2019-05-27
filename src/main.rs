use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;
use sacn::DmxSource;
use schedule_recv;

fn main() -> Result<(), ExitFailure> {
    let tick = schedule_recv::periodic_ms(1);

    loop {
        tick.recv().unwrap();
        let dmx_source = DmxSource::new("Bad Controller").unwrap();
        dmx_source.send_with_priority(1, &[255; 512], 200).unwrap()
    }

    Ok(())
}
