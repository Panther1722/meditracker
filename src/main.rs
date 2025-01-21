use std::fs::File;
use std::io::{self, Write};
use serde::Serialize;
use csv::Writer;

#[derive(Serialize)]
struct Medication {
    name: String,
    schedule: String,
}

#[derive(Serialize)]
struct Patient {
    id: String,
    name: String,
    age: u32,
    gender: String,
    disease: String,
    medications: Vec<Medication>,
}

fn display_menu() {
    println!("\nMediTracker CLI");
    println!("1. Add Patient");
    println!("2. View Patients");
    println!("3. Add Medication");
    println!("4. View Medications");
    println!("5. Remove Medication");
    println!("6. Update Medication");
    println!("7. Remove Patient");
    println!("8. Export to CSV");
    println!("9. Exit");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

fn add_patient(patients: &mut Vec<Patient>) {
    println!("\nEnter patient ID: ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id = id.trim().to_string();

    println!("Enter patient name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    let age: u32 = loop {
        println!("Enter patient age: ");
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).unwrap();
        match age_input.trim().parse() {
            Ok(age) => break age,
            Err(_) => println!("Invalid age. Please enter a valid number."),
        }
    };

    println!("Enter patient gender: ");
    let mut gender = String::new();
    io::stdin().read_line(&mut gender).unwrap();
    let gender = gender.trim().to_string();

    println!("Enter patient disease: ");
    let mut disease = String::new();
    io::stdin().read_line(&mut disease).unwrap();
    let disease = disease.trim().to_string();

    patients.push(Patient {
        id,
        name,
        age,
        gender,
        disease,
        medications: Vec::new(),
    });
    println!("Patient added successfully!");
}

fn view_patients(patients: &Vec<Patient>) {
    if patients.is_empty() {
        println!("\nNo patients found.");
    } else {
        println!("\nPatients:");
        for patient in patients {
            println!(
                "- ID: {}, Name: {}, Gender: {}, Disease: {}",
                patient.id, patient.name, patient.gender, patient.disease
            );
        }
    }
}

fn add_medication(patients: &mut Vec<Patient>) {
    println!("\nEnter the patient ID: ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id = id.trim().to_string();

    if let Some(patient) = patients.iter_mut().find(|p| p.id == id) {
        println!("Enter the medication name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();

        println!("Enter the schedule (e.g., '8 AM, 2 PM, 8 PM'): ");
        let mut schedule = String::new();
        io::stdin().read_line(&mut schedule).unwrap();
        let schedule = schedule.trim().to_string();

        patient.medications.push(Medication { name, schedule });
        println!("Medication added successfully for {}!", patient.name);
    } else {
        println!("Patient not found.");
    }
}

fn view_medications(patients: &Vec<Patient>) {
    for patient in patients {
        println!("\nPatient: {} (ID: {})", patient.name, patient.id);
        if patient.medications.is_empty() {
            println!("  No medications found.");
        } else {
            for med in &patient.medications {
                println!("  - {}: {}", med.name, med.schedule);
            }
        }
    }
}

fn remove_medication(patients: &mut Vec<Patient>) {
    println!("\nEnter the patient ID: ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id = id.trim().to_string();

    if let Some(patient) = patients.iter_mut().find(|p| p.id == id) {
        println!("Enter the medication name to remove: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();

        if let Some(pos) = patient.medications.iter().position(|m| m.name == name) {
            patient.medications.remove(pos);
            println!("Medication '{}' removed successfully from {}!", name, patient.name);
        } else {
            println!("Medication not found for {}.", patient.name);
        }
    } else {
        println!("Patient not found.");
    }
}

fn update_medication(patients: &mut Vec<Patient>) {
    println!("\nEnter the patient ID: ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id = id.trim().to_string();

    if let Some(patient) = patients.iter_mut().find(|p| p.id == id) {
        println!("Enter the medication name to update: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();

        if let Some(med) = patient.medications.iter_mut().find(|m| m.name == name) {
            println!("Enter the new schedule (e.g., '8 AM, 2 PM, 8 PM'): ");
            let mut schedule = String::new();
            io::stdin().read_line(&mut schedule).unwrap();
            let schedule = schedule.trim().to_string();

            med.schedule = schedule;
            println!("Medication '{}' updated successfully!", name);
        } else {
            println!("Medication not found for {}.", patient.name);
        }
    } else {
        println!("Patient not found.");
    }
}

fn remove_patient(patients: &mut Vec<Patient>) {
    println!("\nEnter the patient ID to remove: ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id = id.trim().to_string();

    if let Some(pos) = patients.iter().position(|p| p.id == id) {
        patients.remove(pos);
        println!("Patient with ID '{}' removed successfully!", id);
    } else {
        println!("Patient not found.");
    }
}

fn export_to_csv(patients: &Vec<Patient>) {
    let file_path = "patients.csv";
    match File::create(file_path) {
        Ok(file) => {
            let mut writer = Writer::from_writer(file);

            // Write header
            if writer.write_record(&["Patient ID", "Name", "Age", "Gender", "Disease", "Medications"]).is_err() {
                println!("Failed to write header.");
                return;
            }

            for patient in patients {
                let medications = patient
                    .medications
                    .iter()
                    .map(|m| format!("{} ({})", m.name, m.schedule))
                    .collect::<Vec<String>>()
                    .join("; ");

                if writer
                    .write_record(&[&patient.id, &patient.name, &patient.age.to_string(), &patient.gender, &patient.disease, &medications])
                    .is_err()
                {
                    println!("Failed to write patient data to CSV.");
                }
            }
            println!("Data exported successfully to {}", file_path);
        }
        Err(err) => {
            println!("Failed to create CSV file: {}", err);
        }
    }
}

fn main() {
    let mut patients: Vec<Patient> = Vec::new();

    loop {
        display_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => add_patient(&mut patients),
            "2" => view_patients(&patients),
            "3" => add_medication(&mut patients),
            "4" => view_medications(&patients),
            "5" => remove_medication(&mut patients),
            "6" => update_medication(&mut patients),
            "7" => remove_patient(&mut patients),
            "8" => export_to_csv(&patients),
            "9" => {
                println!("Exiting MediTracker. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }









    
}