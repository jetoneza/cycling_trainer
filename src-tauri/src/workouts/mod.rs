use quick_xml;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WorkoutFile {
    #[serde(rename = "sportType")]
    sport_type: String,
    author: String,
    name: String,
    description: String,
    tags: Tags,
    workout: Workout,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Tags {
    tag: Vec<Tag>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Tag {
    #[serde(rename = "@name")]
    name: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Workout {
    #[serde(rename = "$value")]
    workouts: Vec<WorkoutType>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
enum WorkoutType {
    Warmup {
        #[serde(rename = "@Duration")]
        duration: u16,
        #[serde(rename = "@PowerLow")]
        power_low: f64,
        #[serde(rename = "@PowerHigh")]
        power_high: f64,
        #[serde(rename = "@pace")]
        pace: u32,
        #[serde(rename = "@Cadence")]
        cadence: u8,
    },
    SteadyState {
        #[serde(rename = "@Duration")]
        duration: u16,
        #[serde(rename = "@Power")]
        power: f64,
        #[serde(rename = "@pace")]
        pace: u32,
        #[serde(rename = "@Cadence")]
        cadence: u8,
    },
    Cooldown {
        #[serde(rename = "@Duration")]
        duration: u16,
        #[serde(rename = "@PowerLow")]
        power_low: f64,
        #[serde(rename = "@PowerHigh")]
        power_high: f64,
        #[serde(rename = "@pace")]
        pace: u32,
        #[serde(rename = "@Cadence")]
        cadence: u8,
    },
}

pub fn get_workouts() {
    let xml = r#"
        <workout_file>
            <author>J.Ordaneza</author>
            <name>Z2 Chiller - 30mins</name>
            <description>Zone 2 Workout for 30 minutes.</description>
            <sportType>bike</sportType>
            <tags>
                <tag name="z2"/>
            </tags>
            <workout>
                <Warmup Duration="360" PowerLow="0.39908534" PowerHigh="0.61249995" pace="1428186484" Cadence="60"/>
                <SteadyState Duration="180" Power="0.67347556" pace="1428186484" Cadence="75"/>
                <SteadyState Duration="180" Power="0.67347556" pace="1428186484" Cadence="85"/>
                <SteadyState Duration="60" Power="0.67347556" pace="1428186484" Cadence="100"/>
                <SteadyState Duration="180" Power="0.64908534" pace="0" Cadence="75"/>
                <SteadyState Duration="180" Power="0.64908534" pace="0" Cadence="85"/>
                <SteadyState Duration="60" Power="0.64908534" pace="0" Cadence="100"/>
                <SteadyState Duration="180" Power="0.64908534" pace="0" Cadence="75"/>
                <SteadyState Duration="180" Power="0.64908534" pace="0" Cadence="85"/>
                <SteadyState Duration="60" Power="0.64908534" pace="0" Cadence="100"/>
                <SteadyState Duration="180" Power="0.61249995" pace="0" Cadence="75"/>
                <SteadyState Duration="180" Power="0.61249995" pace="0" Cadence="85"/>
                <SteadyState Duration="60" Power="0.61249995" pace="0" Cadence="100"/>
                <Cooldown Duration="360" PowerLow="0.61249995" PowerHigh="0.39908534" pace="1428186484" Cadence="60"/>
            </workout>
        </workout_file>
    "#;

    let workout: WorkoutFile = quick_xml::de::from_str(xml).unwrap();

    println!("{:?}", workout)
}
