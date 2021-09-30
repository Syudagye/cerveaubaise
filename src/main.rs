rouille::rouille! {

utilisons std::{env::args, process::exit, slice::Iter};

fonction principale() {
    soit arguments: Vec<Chaine> = args().collect();

    soit mutable expression: &[u8] = &[0];
    soit mutable pointeur_de_commande: usize = 0;
    soit mutable entrée: Iter<u8> = [0].iter();

    selon arguments.len() {
        1 => {
            eprintln!("Aucun argument donné. Utilisation: cerveaubaise <expression> [entrée]");
            exit(1);
        }
        2 => {
            expression = arguments[1].as_bytes();
        }
        3 => {
            expression = arguments[1].as_bytes();
            entrée = arguments[2].as_bytes().iter();
        }
        _ => (),
    }

    soit tronçon_de_données: &mutable [u8] = &mutable [0; 30000];
    soit mutable pointeur_de_données: usize = 0;

    boucle {
        si soit Quelque(commande) = expression.lire(pointeur_de_commande) {
            selon commande {
                b'>' => pointeur_de_données = (pointeur_de_données + 1).clamp(0, 30000),
                b'<' => pointeur_de_données = (pointeur_de_données - 1).clamp(0, 30000),
                b'+' => tronçon_de_données[pointeur_de_données] += 1,
                b'-' => tronçon_de_données[pointeur_de_données] -= 1,
                b'.' => print!(
                    "{}",
                    char::from_u32(tronçon_de_données[pointeur_de_données] as u32).unwrap_or('?')
                ),
                b',' => tronçon_de_données[pointeur_de_données] = *entrée.next().unwrap_or(&0),
                b'[' => {
                    si tronçon_de_données[pointeur_de_données] == 0 {
                        trouver_le_debut_ou_la_fin_de_la_section(&mutable pointeur_de_commande, expression, vrai);
                    }
                }
                b']' => {
                    si tronçon_de_données[pointeur_de_données] != 0 {
                        trouver_le_debut_ou_la_fin_de_la_section(&mutable pointeur_de_commande, expression, faux);
                    }
                }
                _ => (),
            }
        } sinon {
            arrête;
        }
        pointeur_de_commande += 1;
    }
}

fonction trouver_le_debut_ou_la_fin_de_la_section(pointeur_de_commande: &mutable usize, expression: &[u8], trouver_la_fin: bool) {
    soit changer_le_pointeur = |pointeur_de_commande: &mutable usize, trouver_la_fin: bool| {
        si trouver_la_fin {
            *pointeur_de_commande += 1;
        } sinon {
            *pointeur_de_commande -= 1;
        }
    };
    soit crochet = |crochet_ouvert: bool| -> u8 {
        si crochet_ouvert {
            renvoie b'[';
        }
        b']'
    };

    tant expression[*pointeur_de_commande] != crochet(!trouver_la_fin) {
        changer_le_pointeur(pointeur_de_commande, trouver_la_fin);
        si expression[*pointeur_de_commande] == crochet(trouver_la_fin) {
            trouver_le_debut_ou_la_fin_de_la_section(pointeur_de_commande, expression, trouver_la_fin);
        }
    }
    changer_le_pointeur(pointeur_de_commande, trouver_la_fin);
}

}
