/// Ce projet permet de lancer un questionnaire de 3 questions choisi par l'utilisateur et de lui doner son score à la fin.

extern crate rand;
use std::io;
use rand::Rng;

/// Tests Unitaires effectué sur la fonction questionnaire
#[cfg(test)]
mod tests {
    #[test]
    fn questionnaire1() {
        let input1 = 1;
        let bonnereponse1 = 3;
    
        assert!(input1 != bonnereponse1);

}

    #[test]
    fn questionnaire2() {
            let input = 3;
            let bonnereponse = 3;
        
            assert!(input == bonnereponse);

    }

    #[test]
    fn questionnaire3() {
        for i in 0..3{
            let bonnereponse = 3;
            assert!(i != bonnereponse);
        } 
    }

    #[test]
    fn questionnaire4() {
        for i in 0..3{
            let input = 3;
            let bonnereponse = 3;
            assert!(input == bonnereponse);
        } 

    }

}

fn main() {

    ///Declaration des differentes matieres au choix de l'utilisateur
    let mut matieres = ["Matematiques", "Histoire", "Musique", "Informatique"];

    /// Declarations des questions, reponses etbonne réponses aux differents questionnaires
    let mut questionHistoire = ["Quelle est la date de la fin de la Première Guerre mondiale ?", "Quand à été mis à disposition des femmes la première pilule contraceptive ?", "De quand date la fondation de la Chine communiste ?"];
    let mut reponseHistoire = [["1914", "1918", "1917"], ["1960","1970","1950"], ["1920","1950", "1949"]];
    let mut bonnesReponsesHistoire=[1,0,2];

    let mut questionMusique = ["Quel groupe pop américain des années 1960 a créé le «son surfin» ?", "Quel chanteur était connu entre autres comme «The King of Pop» et «The Gloved One» ?", "Quelle pop star américaine a eu le succès des charts 2015 avec les singles `` Sorry '' et `` Love Yourself '' ?"];
    let mut reponseMusique = [["The Jackson 5", "The Beatles", "Beach Boys"], ["Freddie Mercury","Michael Jackson","Lou Bega"], ["Justin Bieber","Selena Gomez", "Avril Lavigne"]];
    let mut bonnesReponsesMusique=[2,1,0];

    let mut questionMaths = ["A quel nombre décimal est égal à 3/6 + 6/3 ?", "Lequel de ces angles est plus petit qu'un angle droit ?", "Combien y a-t-il de dizaines dans 23 450 ?"];
    let mut reponseMaths = [["2.5", "3.5", "4.5"], ["Un angle obtus","Un angle aigu","Un angle plat"], ["2 345","234", "23"]];
    let mut bonnesReponsesMaths=[0,1,0];

    let mut questionInfo = ["Lequel des langages informatiques suivants est utilisé pour l'intelligence artificielle ?", "Comment se nomme le cerveau de tout système informatique ?", "Que signifie ASCII ?"];
    let mut reponseInfo = [["C", "PROLOG", "FORTRAN"], ["CPU","Unité arithmétique et logique – ALU","Mémoire"], ["American security code for information interchange","All purpose scientific code for information interchange", "American standard code for information interchange"]];
    let mut bonnesReponsesInfo=[1,0,2];

    ///Presentation à l'utilisateur lors du ```cargo run```
    println!("Bienvenu sur le Quizz de Marie & Julien :");
    println!("Choisisez la matiere :");

    for l in 0..4 {
        println!("{} : {} ",l, matieres[l]);
    }

    /// Gestion de l'input de l'utilisateur
    let mut inputMatiere = String::new();
    io::stdin().read_line(&mut inputMatiere).unwrap();
    let matiereChoisie: i32 = inputMatiere.trim().parse().unwrap();

    /// Gestion du choix de la matiere
        match matiereChoisie{
            0=>questionnaire(questionMaths, reponseMaths, bonnesReponsesMaths),
            1=>questionnaire(questionHistoire,reponseHistoire,bonnesReponsesHistoire),
            2=>questionnaire(questionMusique, reponseMusique, bonnesReponsesMusique),
            3=>questionnaire(questionInfo,reponseInfo,bonnesReponsesInfo),
            /// Gestion des erreurs de l'input
            _=>println!("La matiere que vous avez choisit n'existe pas !"),
            
          }

    
}

/// Fonction generique questionnaire qui se lance en fonction du choix utilisateur en prenant en paramettre les questions, reponses, bonne reponses de la matiere choisie
fn questionnaire(Q:[&str;3], R:[[&str;3];3],BRep:[i32;3]) {
    /// Declaration d'un tableau de stokage des differents indices deja passés, chaques indices est ajouté au tableau a chaque tour de boucle
    let mut arr = [5;3];
    /// Declaration du score de l'utilisateur initialisé à 0
    let mut score=0;
    
    /// Lancement des 3 questions de manieres aléatoire
    for j in 0..3{
        let mut nombre_aleatoire = rand::thread_rng().gen_range(0..3);

            /// Gestion de la non repetition des questions grace au tableau de stockage
            while nombre_aleatoire == arr[0] || nombre_aleatoire == arr[1] || nombre_aleatoire == arr[2]{
                nombre_aleatoire = rand::thread_rng().gen_range(0..3);
            }
            /// Declaration des variables question, reponse et bonne reponses suite au nombre aléatoire défini
            let question = Q[nombre_aleatoire];
            let reponse=R[nombre_aleatoire];
            let bonnesReponses=BRep[nombre_aleatoire];
            
            
            /// Affichage à l'utilisateur des questions
            println!("Question : {} ", question);

            /// Affichage des differentes reponses possible à l'utilisateur
            for i in 0..3 {
                println!("{} : {} ",i, reponse[i]);
            }

            
            println!("Entrez la réponse :");
            
            /// Gestion de l'input del'utilisateur
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let n: i32 = input.trim().parse().unwrap();

            /// Gestion de la réponse donnée par l'utilisateur, modification du score en fonction et affichage de la bonne réponse si l'utilisateur c'est trompé
            
            if n == bonnesReponses && n>=0 && n<3{
                println!("Bonne réponse !");
                score += 1;
                
            }else if n>=0 && n<3{
                println!("Loupé");
                println!("la bonne réponse était: {} ", bonnesReponses);
            }else {
                /// Gestion des erreurs de l'input
                println!("Votre réponse n'existe pas :( ");
            }

            arr[j] = nombre_aleatoire;
        
    }
    /// Affichage du score à l'utilisateur
    println!("Votre score est: {}/3", score);
}
