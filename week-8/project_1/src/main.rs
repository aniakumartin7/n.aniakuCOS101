fn main() {
    let public_servant = vec![
        "Intern",
        "Administrator",
        "Senior Administrator",
        "Office Manager",
        "Director",
        "CEO",
    ];

    let academic = vec![
        "-",
        "Research Assistant",
        "PhD Candidate",
        "Post-Doc Researcher",
        "Senior Lecturer",
        "Dean",
    ];

    let lawyer = vec![
        "Paralegal",
        "Junior Associate",
        "Associate",
        "Senior Associate 1-2",
        "Senior Associate 3-4",
        "Partner",
    ];

    let teacher = vec![
        "Placement",
        "Classroom Teacher",
        "Senior Teacher",
        "Leading Teacher",
        "Deputy Principal",
        "Principal",
    ];

    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    println!("APS Level Checker");
    println!("1. Public Servant");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    let job = 3;
    let experience = 6;

    let mut level = 0;

    if experience >= 1 && experience <= 2 {
        level = 0;
    } else if experience >= 3 && experience <= 5 {
        level = 1;
    } else if experience >= 5 && experience <= 8 {
        level = 2;
    } else if experience >= 8 && experience <= 10 {
        level = 3;
    } else if experience >= 10 && experience <= 13 {
        level = 4;
    } else if experience > 13 {
        level = 5;
    }

    println!("Years of experience: {}", experience);
    println!("Level: {}", levels[level]);

    if job == 1 {
        println!("Position: {}", public_servant[level]);
    } else if job == 2 {
        println!("Position: {}", academic[level]);
    } else if job == 3 {
        println!("Position: {}", lawyer[level]);
    } else if job == 4 {
        println!("Position: {}", teacher[level]);
    } else {
        println!("Invalid job selection");
    }
}