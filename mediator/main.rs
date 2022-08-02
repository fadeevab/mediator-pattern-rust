mod railway_station;
mod trains;

use railway_station::RailwayStation;
use trains::{FreightTrain, PassengerTrain};

fn main() {
    let train1 = PassengerTrain::new("Train 1");
    let train2 = FreightTrain::new("Train 2");

    let mut station = RailwayStation::default();

    station.accept(train1);
    station.accept(train2);
    station.depart("Train 1");
    station.depart("Train 2");
    station.depart("Train 3");
}
