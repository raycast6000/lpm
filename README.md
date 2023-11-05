# The LuSR Package Manager
Simple and stupidly fast, just a better NPM.
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
**ALL PACKAGES ARE STORED IN A FOLDER CALLED `lusr_packages`, WHICH IS AUTOMATICALLY CREATED WHEN YOU FIRST ADD A PACKAGE OR INITIALIZE A NEW LUSR PROJECT.**