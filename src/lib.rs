use wasm_bindgen::prelude::*;
use rand::prelude::*;
use rand_distr::{Distribution, Exp, Normal};
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

// ==================== Core Types ====================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Station {
    pub id: u32,
    pub name: String,
    pub code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Segment {
    pub from: u32,
    pub to: u32,
    pub travel_time_minutes: u32,      // base travel time
    pub capacity: u32,                 // trains per hour
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DisruptionType {
    Technical,
    Weather,
    Organizational,
    Infrastructure,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Disruption {
    pub disruption_type: DisruptionType,
    pub severity: f32,           // 0.0 - 1.0
    pub start_time: DateTime<Utc>,
    pub duration_minutes: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Train {
    pub id: u32,
    pub train_type: String,      // "Sapsan", "Lastochka", etc.
    pub current_segment: Option<u32>,
}

// ==================== Events ====================

#[derive(Clone, Debug)]
pub enum Event {
    TrainDeparture {
        train_id: u32,
        station_id: u32,
        time: DateTime<Utc>,
    },
    TrainArrival {
        train_id: u32,
        station_id: u32,
        time: DateTime<Utc>,
    },
    DisruptionStart {
        segment_id: u32,
        disruption: Disruption,
    },
}

impl Eq for Event {}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time() == other.time()
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time().cmp(&self.time()) // BinaryHeap is max-heap, so we reverse
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Event {
    fn time(&self) -> DateTime<Utc> {
        match self {
            Event::TrainDeparture { time, .. } => *time,
            Event::TrainArrival { time, .. } => *time,
            Event::DisruptionStart { disruption, .. } => disruption.start_time,
        }
    }
}

// ==================== Simulation State ====================

#[wasm_bindgen]
pub struct Simulation {
    stations: Vec<Station>,
    segments: Vec<Segment>,
    events: BinaryHeap<Event>,
    current_time: DateTime<Utc>,
    rng: StdRng,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u64) -> Simulation {
        let rng = StdRng::seed_from_u64(seed);
        Simulation {
            stations: vec![],
            segments: vec![],
            events: BinaryHeap::new(),
            current_time: Utc::now(),
            rng,
        }
    }

    pub fn add_station(&mut self, id: u32, name: String, code: String) {
        self.stations.push(Station { id, name, code });
    }

    pub fn add_segment(&mut self, from: u32, to: u32, travel_time_minutes: u32, capacity: u32) {
        self.segments.push(Segment {
            from,
            to,
            travel_time_minutes,
            capacity,
        });
    }

    pub fn schedule_departure(&mut self, train_id: u32, station_id: u32, time: DateTime<Utc>) {
        self.events.push(Event::TrainDeparture {
            train_id,
            station_id,
            time,
        });
    }

    // TODO: Implement full simulation step
    pub fn run_until(&mut self, end_time: DateTime<Utc>) {
        while let Some(event) = self.events.pop() {
            if event.time() > end_time {
                self.events.push(event); // put it back
                break;
            }
            self.current_time = event.time();
            // TODO: handle event
        }
    }
}