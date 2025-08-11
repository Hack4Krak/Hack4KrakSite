# <picture> <source srcset="https://raw.githubusercontent.com/Hack4Krak/Hack4KrakSite/refs/heads/master/.github/assets/banner-light.png" media="(prefers-color-scheme: dark)"/> <img src="https://raw.githubusercontent.com/Hack4Krak/Hack4KrakSite/refs/heads/master/.github/assets/banner-dark.png" /> </picture>

<p align=center>
    <img alt="Issues" src="https://img.shields.io/github/issues-raw/Hack4Krak/Hack4KrakSite?color=ffb900&label=issues&style=for-the-badge" />
    <img alt="Pull Requests" src="https://img.shields.io/github/issues-pr-raw/Hack4Krak/Hack4KrakSite?color=ffb900&label=PRs&style=for-the-badge" />
    <img alt="Contributors" src="https://img.shields.io/github/contributors/Hack4Krak/Hack4KrakSite?color=ffb900&label=contributors&style=for-the-badge" />
    <img alt="Lines" src="https://img.shields.io/endpoint?url=https://ghloc.vercel.app/api/Hack4Krak/Hack4KrakSite/badge?style=flat&logoColor=white&color=ffb900&style=for-the-badge" />
    <img alt="Last commit" src="https://img.shields.io/github/last-commit/Hack4Krak/Hack4KrakSite?color=ffb900&label=last%20commit&style=for-the-badge" />
</p>

**Welcome to the official Hack4Krak website repository!**

Hack4Krak is the largest Capture The Flag (CTF) competition for high schools in Poland, bringing together the brightest
young minds in cybersecurity. This event challenges participants with exciting puzzles and security challenges,
fostering learning and collaboration in the world of ethical hacking!

## 📅 Project status

We managed to host our first event, but website is still **in development**!
We have to complete more features and fix some bugs before we can call it a day.

- ✅ `28/02/2025` - Successfully completed CTF for [31st highschool](https://www.lo31.krakow.pl/) in Kraków
- ✅ `30/05/2025` - CTF for all primary schools in Kraków.
- 🚧 `2026` - CTF for all high schools in Poland. In development

## 🚜 Development

This repository is a monorepo containing the [frontend](frontend/) and [backend](backend/) code for the Hack4Krak
website.
The frontend is built with Nuxt and the backend is built with Rust. The two parts are connected via a REST API.

For documentation refer to specific `DEVELOPMENT.md` files:

- [Frontend](frontend/DEVELOPMENT.md)
- [Backend](backend/DEVELOPMENT.md)

## 🚀 Deployment

The production website is currently fully self-deployed on a VPS with [Dokploy](https://dokploy.com/).
To read more about deployment, env variables and other settings, refer to the [`DEPLOYMENT.md`](DEPLOYMENT.md) file.

You access our frontend at [hack4krak.pl](https://hack4krak.pl/) and backend
at [api.hack4krak.pl](https://api.hack4krak.pl/).

## 🎉 Stats

![Repobeats analytics image](https://repobeats.axiom.co/api/embed/b2a0612285a5cfef1231975dc94e601dc5f0b983.svg "Repobeats analytics image")
