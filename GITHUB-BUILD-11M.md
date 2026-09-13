# Nexus 1.10.3 — compilation Windows automatique

Cette copie contient le workflow GitHub `.github/workflows/build-windows-11m.yml`.

## Utilisation

1. Créer un dépôt GitHub personnel.
2. Copier tout le contenu de ce dossier dans le dépôt.
3. Faire un commit et pousser les fichiers.
4. Dans GitHub : **Actions** → **Build Nexus Windows 11m** → **Run workflow**.
5. Attendre la fin du travail `Build Windows installer (NSIS)`.
6. Ouvrir le résultat du workflow et télécharger l'artifact :
   `Nexus-1.10.3-11m-Windows-Setup`.
7. Décompresser l'artifact : il contient le `*-setup.exe` à installer sous Windows.

Aucun Rust, MSYS2, Node.js ou compilateur n'est à installer sur le PC Windows qui recevra le programme :
la compilation est effectuée par GitHub Actions.

Le workflow télécharge et vérifie le modèle DeepCW nécessaire au projet, installe la chaîne MinGW/NSIS,
puis utilise le script de compilation Windows déjà présent dans le projet.
