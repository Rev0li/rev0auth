# Next-Work - Consolidation de base

## But

Ce document sert de base pour consolider notre futur socle de travail.
L'objectif n'est pas de refaire pour refaire, mais de rendre le projet plus petit, plus lisible, plus modulable et plus facile a reprendre dans le temps.

Learning mode: the project is also used as a Rust onboarding fortress, with commented steps, small commits, and readable history.

Rapport backend priorise: `docs/nest-001-audit-backend.md`

La logique est simple: moins de couplage, plus de frontieres claires, plus de pieces remplaçables.

---

## Lecture franche de l'etat actuel

Le projet a deja une base solide sur l'auth, la securite, les tests, les scripts et la documentation.
Le risque principal maintenant n'est plus la fonctionnalite brute, mais l'empilement: trop de logique dans les memes fichiers, trop de comportements meles, et pas assez de separation entre presentation, interaction et metier.

Pour continuer proprement, il faut consolider au lieu d'etendre au hasard.

---

## Consolidation cible

### 1. Frontend HTML  __ Prochain module nous feront un reset toutal passer en etapes 2.

Le HTML doit redevenir une couche d'assemblage, pas une couche de logique.

Objectifs:
- decouper les pages en blocs simples et stables;
- isoler les sections reutilisables;
- garder des structures previsibles pour chaque vue;
- eviter les templates trop monolithiques.

Regle pratique:
- une page = un role principal;
- un bloc = une responsabilite visuelle ou fonctionnelle;
- pas de markup inutilement profond.

### 2. CSS

Le CSS doit etre reorganise par couches, pas par accumulation.

Objectifs:
- base typographique et tokens communs;
- layout global;
- composants reutilisables;
- variantes et etats;
- responsive a part.

A consolider:
- variables communes;
- nomenclature plus stable;
- suppression des regles redondantes;
- separation nette entre style de structure et style de decoration.

Cible ideale:
- un fichier ou un petit groupe de fichiers par intention;
- pas de gros fichier qui fait tout.

### 3. JavaScript

Le JavaScript doit etre module et orienté actions, pas un bloc de reflexes entassees.

Objectifs:
- extraire les comportements en petites fonctions;
- separer l'initialisation, les event listeners et la logique metier;
- limiter les effets de bord;
- garder les noms explicites.

Pattern souhaitable:
- `dom/` pour la lecture UI;
- `state/` pour la donnees locale;
- `services/` pour les appels ou comportements externes;
- `ui/` pour les actions visuelles;
- un point d'entree clair.

Le but est qu'une modification locale n'oblige pas a rouvrir tout le front.

---

## Consolidation du reste du projet

### Backend   Nouvelle etapes 1

Le backend doit rester la partie qui porte le sens metier et la securite.

A consolider:
- routes plus lisibles;
- services petits et nommes;
- stockage bien isole;
- validation proche des entrees;
- erreurs uniformes.

But: qu'une future feature se branche sans casser l'existant.

### Tests

Le test doit suivre la structure du code, pas la brouiller.

A faire:
- tests unitaires courts et exacts;
- tests d'integration isoles;
- tests lourds separes;
- regroupement par famille;
- execution parallele seulement quand l'isolation le permet.

Lecture pratique:
- `unit` = rapide et frequente;
- `integration` = verification des frontieres;
- `perf` = controle ponctuel.

### Scripts et ops

Les scripts doivent devenir une couche de confiance, pas un sous-projet flou.

A consolider:
- un script par intention claire;
- noms coherents;
- commandes reproductibles;
- pas de logique cachee dans des scripts trop longs;
- documentation minimale mais precise.

### Documentation

La doc doit suivre le code, mais avec un niveau de synthese plus fort.

A consolider:
- une page par grand sujet;
- un rapport de synthese par ticket majeur;
- les choix importants explicitement traces;
- les compromis clairement assumes.

---

## Refactorisation plus petite et plus modulable

### Principe

Si une piece grossit trop, on la coupe avant qu'elle devienne un point de friction.

On cherche a obtenir:
- moins de duplication;
- moins de fichiers verbeux;
- plus de responsabilites uniques;
- plus de fichiers faciles a relire;
- plus de changements localises.

### Ce que ca veut dire concretement

#### HTML
- extraire les sections repetitives;
- garder les templates lisibles;
- separe la structure de la logique.

#### CSS
- regrouper par couches;
- extraire les patterns partages;
- limiter les override sauvages;
- garder une convention stable.

#### JavaScript
- decouper par fonction d'usage;
- un module = un objectif principal;
- eviter les gros fichiers utilitaires fourre-tout;
- brancher les comportements au plus pres du besoin.

#### Tous les autres fichiers
- une seule responsabilite par fichier quand c'est possible;
- un nom qui dit ce que ca fait;
- un dossier qui dit ce qu'il contient;
- un export qui dit ce qui est public.

---

## Rapport de consolidation

### Ce qui est deja bien
- base auth serieuse;
- securite prise au serieux;
- tests deja riches;
- scripts et docs deja presents;
- rythme d'execution fiable.

### Ce qui doit etre consolide
- reduction des gros blocs;
- meilleure isolation front/back;
- meilleure separation presentation/interaction/metier;
- plus de modularite dans le front;
- plus de lisibilite dans les points d'entree.

### Ce qui ne doit pas changer
- la vitesse de livraison;
- le reflexe de verification;
- la logique ticketisee;
- la priorite sur le concret.

### Ce qu'on veut gagner
- un projet plus simple a reprendre;
- un projet plus simple a tester;
- un projet plus simple a modifier;
- un projet plus simple a expliquer;
- un projet plus simple a faire grandir.

---

## Plan de consolidation

### Phase 1 - Audit du socle
- relire les fichiers front;
- identifier les plus gros blocs;
- marquer les duplications;
- noter les frontieres floues.

### Phase 2 - Coupe par responsabilite
- extraire les pieces trop grosses;
- isoler les comportements JS;
- clarifier les couches CSS;
- reduire le poids des templates.

### Phase 3 - Stabilisation
- verifier que chaque refactor reste testable;
- ajuster les tests si besoin;
- documenter les choix majeurs;
- conserver le comportement visible.

### Phase 4 - Consolidation continue
- chaque nouvelle feature doit respecter la modularite;
- chaque gros fichier doit etre suspect;
- chaque repetition doit etre candidate a extraction.

---

## Regles simples pour la suite

1. Ne pas grossir un fichier si une extraction est possible.
2. Ne pas melanger style, comportement et metier.
3. Ne pas garder une structure par habitude si elle peut etre decoupee.
4. Ne pas sacrifier la lisibilite pour gagner une minute.
5. Ne pas refactoriser sans test ou sans intention claire.

---

## Conclusion

La consolidation, ici, ce n'est pas du polish.
C'est la condition pour que le projet reste maitrisable quand il continue a grandir.

Si on fait bien cette phase, on gagne un socle plus propre, plus petit dans la tete, plus fort dans le temps, et plus facile a faire evoluer sans se perdre.
