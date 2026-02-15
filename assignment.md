# Goal
The goal of this assignment is to master the core of continuous integration.
To achieve this goal, the students are asked to implement a small continuous integration CI server.
This CI server will only contain the core features of continuous integration.
 The features are all specified below, as grading criteria.

The grading focuses on the understanding and implementation of the core CI features,
but also considers the application of software engineering on the development process, see the grading scheme below.

## P1
Property (core CI feature #1 - compilation):
- the CI server supports compiling the group project
- a static syntax check is to be performed for languages without compiler.
- Compilation is triggered as webhook, the CI server compiles the branch where the change has been made,
as specified in the HTTP payload.

Assessment: The grader does a commit in a README in a specific branch, and observes on the server's console that compilation is run
(observation can also be made through P3 and P6). Optionally, the grader has a look at the implementation
 or the tests of this feature.

## P2
Property (core CI feature #2 - testing): the CI server supports executing the automated tests of the group project.
Testing is triggered as webhook, on the branch where the change has been made, as specified in the HTTP payload.

Preparation: the students prepare a specific branch called "assessment" (not "master").
The students document in the README how test execution has been implemented and unit-tested.

Assessment: The grader changes the oracle of an assertion in one test,
and observes that tests are executed and at least one fails (the one with the changed assertion).
Optionally, the grader has a look at the implementation or the tests of this feature.

## P3
Property (core CI feature #3 - notification): the CI server supports notification of CI results.
At least one notification mechanism of the following list is implemented:
- Commit status: the CI server sets the commit statusLinks to an external site.
on the repository (REST APILinks to an external site. for GitHub)
- Email: the CI server sends an email to the project member about the build result.

Preparation: The students document in the README how notification has been implemented and unit-tested.

Assessment: The grader observes the status of the change made while assessing P2.
Optionally, the grader may look at the implementation or the tests of this feature.

# To get a  P+: Pass with distinction (at least two properties must be achieved)
## P7
Property (CI feature): the CI server keeps the history of the past builds.
This history persists even if the server is rebooted.
Each build is given a unique URL, that is accessible to get the build information (commit identifier, build date, build logs).
One URL exists to list all builds.

Preparation: the students document the build list URL in the README.

Assessment: The grader opens the build list URL by clicking on it in the README.
The grader randomly clicks on a build and assesses the appropriateness of the information.

## P8
Property: the group is creative and proactive, they have done something remarkable, for which they are proud.

Assessment: the students claim in their "Statement of contributions" that they have done something valuable and remarkable in their project.
The grader subjectively assess whether it counts towards a distinction.
