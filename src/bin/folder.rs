#[derive(Clone, Debug)]
struct School {
    principle: String,
    classrooms: Vec<Classroom>
}

#[derive(Clone, Debug)]
enum Classroom {
    ScienceLab { num_beakers: i32, eyewash: bool},
    EnglishClass { num_books: i32 }
}

trait Folder {
    fn map_school(&self, s: School) -> School {
        School {
            principle: self.map_principle(s.principle),
            classrooms: self.map_classsrooms(s.classrooms)
        }
    }
    fn map_principle(&self, s: String) -> String {
        s
    }
    fn map_classsrooms(&self, classrooms: Vec<Classroom>) -> Vec<Classroom> {
        classrooms.into_iter().map(|classroom| self.map_classroom(classroom)).collect()
     }
    fn map_classroom(&self, room: Classroom) -> Classroom {
        room
    } 
}

struct DoubleTransformedClassrooms;
impl Folder for DoubleTransformedClassrooms {
    fn map_classsrooms(&self, classrooms: Vec<Classroom>) -> Vec<Classroom> {
        classrooms.into_iter().flat_map(|classroom| {
            [classroom.clone(), classroom].map(|classroom| self.map_classroom(classroom))
        }).collect()
    }

    fn map_classroom(&self, room: Classroom) -> Classroom {
        match room {
            Classroom::ScienceLab { num_beakers, eyewash } => Classroom::ScienceLab { num_beakers: num_beakers * 2, eyewash: !eyewash },
            t => t
        }
    }
}


fn main() {
    let school = School {
        principle: "me".into(),
        classrooms: vec![Classroom::ScienceLab { num_beakers: 8, eyewash: true }]
    };
    dbg!(DoubleTransformedClassrooms.map_school(school));
}
