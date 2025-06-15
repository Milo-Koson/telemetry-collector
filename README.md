# Telemetry-Collector (🛠️ Projet DevOps)

## 📌 Objectif du projet

Ce projet a été conçu comme un **exercice d'entraînement aux pratiques DevOps modernes**. J'ai combiné le développement d'une application Rust avec la mise en place d'une stack de supervision (Prometheus + Grafana), dans une optique de monitoring, de containerisation et d'automatisation.

Ce projet m'a permis plusieurs choses :

- Booster mes connaissances en **Rust** ainsi qu'en orchestration de services via **Docker Compose**
- Apprendre à collecter de métriques via **Prometheus**
- Afficher des dashboards avec **Grafana**
- L'orchestration de services via **Docker Compose**
- Concevoir une **pipeline CI/CD** (vérification de format, tests, ...) pour un projet **Rust**

---

## ⚙️ Fonctionnalités du projet

L'application expose des **métriques systèmes** via un endpoint `/metrics` au format **Prometheus**, incluant :

- L’utilisation CPU
- L’utilisation mémoire
- Le flux réseau

Ces métriques sont ensuite **scrapées par Prometheus** et **visualisées dans Grafana** via des dashboards personnalisés.

---

## 🚀 Lancement du projet

### 🧱 Prérequis

- Docker & Docker Compose installés
- Rust
- Cargo

Il y a 2 manières de démarrer le projet :

### Ⓜ️ Démarrage avec le Makefile

À la racine du projet, il y a un **Makefile** qui peut effectuer 5 commandes sur le fichier ```docker/docker-compose.yml```:

- Make up : Permet de lancer la commande ```docker compose -f up -d``` (sans le build des modifications)
- Make up-build : Permet de lancer la commande ```docker compose -f up -d --build``` (avec le build des modifications)
- Make down : Permet de lancer la commande ```docker compose -f down```
- Make logs : Permet de lancer la commande ```docker compose -f logs -f```
- Make clean : Permet de lancer la commande ```docker compose down``` + Suppression des volumes + Suppression des networks

### ▶️ Démarrage avec Docker

Depuis la racine du projet :

```bash
cd docker
docker-compose up --build
```

Cela va :

- Build l’image de l’application
- Démarrer l’application **Telemetry**, **Prometheus** et **Grafana**
- Exposer les ports suivants :
  
  - 8080 : Telemetry
  - 9090 : Prometheus
  - 3000 : Grafana
    - Identifiant : admin
    - Mot de passe : admin
    - **⚠️ La sécurité n'est pas l'objectif de ce projet ! ⚠️**

### 🖥️ Accès aux interfaces

- Enpoint Telemetry : ```http://localhost:8080```
- Grafana : ```http://localhost:3000```
- Prometheus : ```http://localhost:9090```

Il est également possible de modifier le port d'écoute et le chemin du endpoint d'exposition des métriques en configurant les variables d'environnement suivantes :

- **TELEMETRY_PORT** (type u16, par exemple 8081)

- **TELEMETRY_METRICS_PATH** (type String, par exemple "/my_metrics")

Ces paramètres sont gérés dynamiquement dans le code.

### 📊 Grafana

Dans le répertoire ```docker/supervision/grafana``` j'ai pu provisionner grafana avec 2 dashboards stockés dans un volume docker afin qu'ils soient inclus par défaut lorsque l'utilisateur se connecte.

- Le 1er dashboard affiche l'usage du CPU en fonction du temps (Données rafraîchies toutes les 5 secondes).
- Le 2e dashboard affiche le taux d'espace libre et le taux d'espace occupé sur le disque de stockage.

### 🔄 Intégration Continue (CI)

J'ai utilisé **GitHub Actions** pour automatiser les vérifications à chaque modification de la branche main.

Le pipeline exécute les étapes suivantes :

- Lint avec Clippy : Analyse statique du code, échoue en cas d’avertissements.
- Compilation : Construction du projet avec ```cargo build```.
- Tests : Exécution des tests unitaires et d’intégration avec ```cargo test```.
- Audit de sécurité : Vérification des dépendances connues vulnérables via ```cargo audit```.

Chaque étape est conçue pour garantir la qualité, la sécurité et la stabilité du code.
