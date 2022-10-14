# TODO

Ideas and thoughts that need to be organized

## Git

Students need to understand Git very early. A lesson on git basics is important

- Explain Git
- Create a github account
- Create a repository
- Connect the repository locally
- Understand branches i.e. main/master
- Create a branch, change to the branch, edit a file, commit, push, create a PR, review, and merge

## Markdown

Understand basic markdown syntax. Helps with documentation, i.e README.md files. Any `.md` file.

## Documentation

Documentation is super important. It's not just for others, it's for your future self. You will forget how something works. You will forget the reasons why you did something a certain way. In a world where remote working is increasingly common, documentation is mandatory. A sign of a good employee, a senior employee, is someone that understands that there is more to code than code. New employees need to be able to understand existing code quickly, so that they can contribute faster. When people leave a company for a better opportunity or personal reasons, you need other people less familiar with a corner of the code base to be able to jump in and maintain and extend.

## Versions

`major.minor.patch`, i.e. `1.2.56` would mean a major version of `1`, a minor version of `2` and a patch of `56`. This is called semantic versioning. The intention of a major version is to indicate if there are "breaking changes", meaning that they introduced a change that will require you to change how you are currently using it. Meaning, if you upgrade without making changes, your system will stop working as expected. A patch is typically a bug fix, meaning something was broken or there was a security concern, or a dependency of that software was updated. A lot of software updates are security related and this is a major reason why it's imporant to keep all software and tools up to date. A minor version increase is (should-be) non-breaking, in that it should not affect existing uses. It should be safe to upgrade any minor and patch versions. Not everybody that increases version numbers follows the rules correctly, so there are no guarantees. When updating dependencies in your code, you should update the dependencies on a separate branch and test in a non-production environment prior to merging the updates into the main branch. Major version upgrades almost always need to be planned and performed carefully.

## Maintenance

Keep everything up to date. Keep tools up to date. Packages/librares, etc. It is common for something to break when upgrading software. All software has release notes in the form of a `CHANGELOG.md`, `RELEASE_NOTES.md`, or similar in their source code repository. Figuring out what broke is much easier when you're only performing a single upgrade on a single dependency. When you're very out of date and are upgrading major versions

## Bugs

Errors in the code. A bug will produce a different output than expected. There are seemingly limitless paths that a request can take through your code

## Testing

There are many types of testing: unit, integration, load, stress, canary, etc. Tests are written in code that trigger paths through your code. They validate that the output is the expected value. Tests are important because they are permanent guardians against regression bugs. You can re-run tests after every change to ensure that your change didn't break anything that worked previously. Bugs that are caused by code changes are called regression bugs. Sometimes this is referred to as regression testing. 

Functional tests and non-functional tests - functional tests verify the functionality. Non-functional tests verify that the software has other qualities such as performance, security, and scalability. Scalability is related to load testing. Load testing verifies that the software can handle an increase in load / traffic / requests. Code intended to be made available to the world should consider it's ability to service hundreds, thousands, and millions of users simultaneously (that's a good problem to have if you have that many users - it means your product is successful).

## Analogies

I need to identify as many good analogies as possible to make this easy. I should hire an illustrator to illustrate these concpets for easier consumption.

- Git: Progress points in some games. You store your progress. You can always add more progress. Or, if you mess up, you can go back to a previously saved point. 

- Docker: Some games are only available on Xbox or playstation. Docker would be similar to having an emulator on your gaming system that allows any game to run. Your console may not run the game natively, but the emulator (Docker) is able to run anything. This analogy is generic and inaccurate, but the intent is to understand that Docker is a tool that allows the loading and starting of software (or a game in this analogy) to run on anybodies computer (gaming system in this analogy), regardless of what computer (operating system) they own.

- Terminal: On your computer or laptop you have what is called a graphical user interface or GUI (pronounced "gooey"). That is what you're looking at. There are windows, tabs, your cursor, icons, menus, etc. Before the GUI, computers only offered the command line interface or CLI (Each letter pronounced individually). To see the folders (more accurately called "directories") you had to type something like `dir` to list the directories. (link to dos docker container: https://github.com/jgoerzen/docker-dosemu)