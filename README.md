# The LuSR Package Manager
Simple and stupidly fast, just a better NPM.
# Installation
The package manager is not finished yet, but you can still install it on [Windows](https://github.com/raycast6000/lpm/releases/latest).
# Quick documentation
Make sure that before installing or uninstalling packages you are in the same directory as your project.
### Initializing a LuSR project
This creates a folder `example_project` and some necessary files inside it.
```bash
lpm init example_project
```
### Installing packages
```bash
lpm add package1 package2 package3
```
### Uninstalling packages
```bash
lpm rm package1 package2 package3
```
### Publishing/Updating a package
Before publishing packages you must be authenticated, to do so run:
```bash
lpm auth your_token
```
Replace `your_token` with your token, which you can create in our website.
Make sure that before publising your package you've added a LICENSE and your contact info inside
config.json. \
Now that you're all set, run:
```bash
lpm publish
```
**NOTE: Same thing when updating, just run that command.**

**ALL PACKAGES ARE STORED IN A FOLDER CALLED `lusr_packages`, WHICH IS AUTOMATICALLY CREATED WHEN YOU FIRST ADD A PACKAGE OR INITIALIZE A NEW LUSR PROJECT.**