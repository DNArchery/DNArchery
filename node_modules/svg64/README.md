[![Travis CI](https://travis-ci.com/scriptex/svg64.svg?branch=master)](https://travis-ci.com/scriptex/svg64)
[![Github Build](https://github.com/scriptex/svg64/workflows/Build/badge.svg)](https://github.com/scriptex/svg64/actions?query=workflow%3ABuild)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/34d3d75710534dc6a38c3584a1dcd068)](https://www.codacy.com/gh/scriptex/svg64/dashboard?utm_source=github.com&utm_medium=referral&utm_content=scriptex/svg64&utm_campaign=Badge_Grade)
[![Codebeat Badge](https://codebeat.co/badges/d765a4c8-2c0e-44f2-89c3-fa364fdc14e6)](https://codebeat.co/projects/github-com-scriptex-svg64-master)
[![CodeFactor Badge](https://www.codefactor.io/repository/github/scriptex/svg64/badge)](https://www.codefactor.io/repository/github/scriptex/svg64)
[![DeepScan grade](https://deepscan.io/api/teams/3574/projects/5257/branches/40799/badge/grade.svg)](https://deepscan.io/dashboard#view=project&tid=3574&pid=5257&bid=40799)
[![Analytics](https://ga-beacon-361907.ew.r.appspot.com/UA-83446952-1/github.com/scriptex/svg64/README.md?pixel)](https://github.com/scriptex/svg64/)

# SVG64

> Convert SVG to base64 anywhere

## Visitor stats

![GitHub stars](https://img.shields.io/github/stars/scriptex/svg64?style=social)
![GitHub forks](https://img.shields.io/github/forks/scriptex/svg64?style=social)
![GitHub watchers](https://img.shields.io/github/watchers/scriptex/svg64?style=social)
![GitHub followers](https://img.shields.io/github/followers/scriptex?style=social)

## Code stats

![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/scriptex/svg64)
![GitHub repo size](https://img.shields.io/github/repo-size/scriptex/svg64?style=plastic)
![GitHub language count](https://img.shields.io/github/languages/count/scriptex/svg64?style=plastic)
![GitHub top language](https://img.shields.io/github/languages/top/scriptex/svg64?style=plastic)
![GitHub last commit](https://img.shields.io/github/last-commit/scriptex/svg64?style=plastic)

## About

If you, like me, are using lots of SVGs when developing, you might have come to a point where you need your SVG used as background image or embedded in your javascript file. The only way to do this is to convert your SVG file to a base64 string and then use it where needed. This package does exactly this - it converts SVGs to base64.

This package works in a browser and in a Node environment. Please read along to understand how.

## Install

```sh
npm i svg64
```

or

```sh
yarn add svg64
```

or

just download this repository and use the files located in `dist` folder

or unclude it from unpkg.com

```html
<script src="https://unpkg.com/svg64"></script>
```

## Usage

### In a browser

```javascript
import svg64 from 'svg64'; // or use window.svg64.default

// This is your SVG DOM element
const svg = document.getElementById('svg');

// This is your DOM element that needs SVG background image
const demo = document.getElementById('demo');

// This is your SVG in base64 representation
const base64fromSVG = svg64(svg);

// Add the base64 image as a background to your element
demo.style.backgroundImage = `url(${base64fromSVG})`;
```

### In Node

```javascript
// Require svg64
const svg64 = require('svg64');

// Import `readFileSync` from the file system module
const { readFileSync } = require('fs');

// Read your SVG file's contents
const svg = readFileSync('./file.svg', 'utf-8');

// This is your SVG in base64 representation
const base64fromSVG = svg64(svg);
```

## Demo

There is a simple demo illustrating how to use the SVG64 module in a browser.

Check it out [here](https://svg64.atanas.info)

There is a simple demo illustrating how to use the SVG64 module in NodeJS.

Check it out [here](https://github.com/scriptex/svg64/blob/master/demo/scripts/node.js)

## LICENSE

MIT

---

<div align="center">
    Connect with me:
</div>

<br />

<div align="center">
    <a href="https://atanas.info">
        <img src="https://raw.githubusercontent.com/scriptex/socials/master/styled-assets/logo.svg" height="20" alt="">
    </a>
    &nbsp;
    <a href="mailto:hi@atanas.info">
        <img src="https://raw.githubusercontent.com/scriptex/socials/master/styled-assets/email.svg" height="20" alt="">
    </a>
    &nbsp;
    <a href="https://www.linkedin.com/in/scriptex/">
        <img src="https://raw.githubusercontent.com/scriptex/socials/master/styled-assets/linkedin.svg" height="20" alt="">
    </a>
    &nbsp;
    <a href="https://github.com/scriptex">
        <img src="https://raw.githubusercontent.com/scriptex/socials/master/styled-assets/github.svg" height="20" alt="">
    </a>
    &nbsp;
    <a href="https://gitlab.com/scriptex">
        <img src="https://raw.githubusercontent.com/scriptex/socials/master/styled-assets/gitlab.svg" height="20" alt="">
    </a>
    &nbsp;
    <a href="https://twitter.com/scriptexbg">
        <img src="https://raw.githubusercontent.com/scriptex/socials/master/styled-assets/twitter.svg" height="20" alt="">
    </a>
    &nbsp;
    <a href="https://www.npmjs.com/~scriptex">
        <img src="https://raw.githubusercontent.com/scriptex/socials/master/styled-assets/npm.svg" height="20" alt="">
    </a>
    &nbsp;
    <a href="https://www.youtube.com/user/scriptex">
        <img src="https://raw.githubusercontent.com/scriptex/socials/master/styled-assets/youtube.svg" height="20" alt="">
    </a>
    &nbsp;
    <a href="https://stackoverflow.com/users/4140082/atanas-atanasov">
        <img src="https://raw.githubusercontent.com/scriptex/socials/master/styled-assets/stackoverflow.svg" height="20" alt="">
    </a>
    &nbsp;
    <a href="https://codepen.io/scriptex/">
        <img src="https://raw.githubusercontent.com/scriptex/socials/master/styled-assets/codepen.svg" width="20" alt="">
    </a>
    &nbsp;
    <a href="https://profile.codersrank.io/user/scriptex">
        <img src="https://raw.githubusercontent.com/scriptex/socials/master/styled-assets/codersrank.svg" height="20" alt="">
    </a>
    &nbsp;
    <a href="https://linktr.ee/scriptex">
        <img src="https://raw.githubusercontent.com/scriptex/socials/master/styled-assets/linktree.svg" height="20" alt="">
    </a>
</div>

---

<div align="center">
Support and sponsor my work:
<br />
<br />
<a href="https://twitter.com/intent/tweet?text=Checkout%20this%20awesome%20developer%20profile%3A&url=https%3A%2F%2Fgithub.com%2Fscriptex&via=scriptexbg&hashtags=software%2Cgithub%2Ccode%2Cawesome" title="Tweet">
	<img src="https://img.shields.io/badge/Tweet-Share_my_profile-blue.svg?logo=twitter&color=38A1F3" />
</a>
<a href="https://paypal.me/scriptex" title="Donate on Paypal">
	<img src="https://img.shields.io/badge/Donate-Support_me_on_PayPal-blue.svg?logo=paypal&color=222d65" />
</a>
<a href="https://revolut.me/scriptex" title="Donate on Revolut">
	<img src="https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/scriptex/scriptex/master/badges/revolut.json" />
</a>
<a href="https://patreon.com/atanas" title="Become a Patron">
	<img src="https://img.shields.io/badge/Become_Patron-Support_me_on_Patreon-blue.svg?logo=patreon&color=e64413" />
</a>
<a href="https://ko-fi.com/scriptex" title="Buy Me A Coffee">
	<img src="https://img.shields.io/badge/Donate-Buy%20me%20a%20coffee-yellow.svg?logo=ko-fi" />
</a>
<a href="https://liberapay.com/scriptex/donate" title="Donate on Liberapay">
	<img src="https://img.shields.io/liberapay/receives/scriptex?label=Donate%20on%20Liberapay&logo=liberapay" />
</a>

<a href="https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/scriptex/scriptex/master/badges/bitcoin.json" title="Donate Bitcoin">
	<img src="https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/scriptex/scriptex/master/badges/bitcoin.json" />
</a>
<a href="https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/scriptex/scriptex/master/badges/etherium.json" title="Donate Etherium">
	<img src="https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/scriptex/scriptex/master/badges/etherium.json" />
</a>
<a href="https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/scriptex/scriptex/master/badges/shiba-inu.json" title="Donate Shiba Inu">
	<img src="https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/scriptex/scriptex/master/badges/shiba-inu.json" />
</a>
</div>
