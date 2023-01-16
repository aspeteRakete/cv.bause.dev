use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ExperienceProps {
}

#[function_component(Experience)]
pub fn experience(_props: &ExperienceProps) -> Html {
    html! {
        <div class="row">
            <div class="col-12 col-lg-8 col-xl-9 pr-0 pr-lg-5">
                <div class="work-section">
                    <h3 class="section-heading">
                        {"Work Experience"}
                    </h3>
                        <div class="work-item py-3">
                            <div class="row align-items-center">
                                <h4 class="work-item-heading col-12 col-md-6 col-lg-6">
                                    {"Technical Lead"}
                                </h4>
                                <div class="work-item-time col-12 col-md-6 col-lg-6 text-muted text-left text-md-right">
                                    {"Hanko GmbH | 2019 - Present"}
                                </div>
                            </div>
                            {"At "} <a href={"//hanko.io"}>{"Hanko"}</a> {" we develop an authentication system based on FIDO2. The goal is to make passwords obsolete and enhance security as well as the user experience when logging in online. \
                            We are currently working on the "}<a href={"//github.com/teamhanko/hanko"}>{"hanko"}</a> {" open-source authentication solution. This software is offered as a SaaS product with additional features. \
                            I had a strong impact by developing and defining the software stack and cloud architecture, developing the roadmap with the CEO, talking with customers and mentoring the development team. \
                            Some of the key components we developed are:"}
                            <ul>
                                <li>{"Hanko Authentication API - golang based FIDO authentication server"}</li>
                                <li>{"Hanko Identity - Full fledged identity solution with authentication, identity management, authorization and OIDC connection."}</li>
                                <li>{"kubernetes infrastructure running on AWS and mangaged with kops"}</li>
                                <li>{"hanko operator - Using the kubernetes operator framework to manage the SaaS products and let clients spawn isolated instances via a web-console"}</li>
                            </ul>
                        </div>

                        <div class="work-item py-3">
                            <div class="row align-items-center">
                                <h4 class="work-item-heading col-12 col-md-6 col-lg-6">
                                    {"Lead Software Developer"}
                                </h4>
                                <div class="work-item-time col-12 col-md-6 col-lg-6 text-muted text-left text-md-right">
                                    {"li - Light Instruments GmbH | 2015 - 2019"}
                                </div>
                            </div>
                            {"At "}<a href={"//light-instruments.de"}>{"light instruments "}</a> {" we developed a hardware/software combination that enhanced beamers with capabilities of video mapping. \
                            In addition we implemented a content management system to schedule Images and Videos in a digital signage context. \
                            As the first employee I was lucky to be part of the creation of this company from the beginning and implemented a very broad set of components and functionalities. \
                            Including but not limited to:"}
                            <ul>
                                <li>{"C++ graphics core utilizing OpenGL and GStreamer"}</li>
                                <li>{"Golang server for scene and video configuration from IPad App and CMS"}</li>
                                <li>{"Configuring custom Linux and provisionig toolkit (automate the creation of machines)"}</li>
                                <li>{"CMS golang server and AWS server infrastucture with ansible"}</li>
                                <li>{"Incremental over-the-air update mechanism with multi version boot and rollback mechanism"}</li>
                            </ul>
                        </div>

                        <div class="work-item py-3">
                            <div class="row align-items-center">
                                <h4 class="work-item-heading col-12 col-md-6 col-lg-6">
                                    {"Research Assistant - Software Challenge"}
                                </h4>
                                <div class="work-item-time col-12 col-md-6 col-lg-6 text-muted text-left text-md-right">
                                    {"Christian Albrechts Universität Kiel - Department of Technical Computer Science | 2011 - 2014"}
                                </div>
                            </div>
                            {"The "}<a href={"//software-challenge.de/"}>{"Software Challenge"}</a>{" is an annual programming competition in which students write a player for a yearly changing board game. \
                            Their programs compete against each other and the winners are awarded a scholarship for studying computer science. \
                            I implemented the server side mechanics for a game and assisted teachers in schools to train the fundamentals of programming in Java."}
                        </div>

                        <div class="work-item py-3">
                            <div class="row align-items-center">
                                <h4 class="work-item-heading col-12 col-md-6 col-lg-6">
                                    {"Research Assistant - Exercise Group Leader"}
                                </h4>
                                <div class="work-item-time col-12 col-md-6 col-lg-6 text-muted text-left text-md-right">
                                    {"Christian Albrechts Universität Kiel - Department of Technical Computer Science | 2009 - 2010"}
                                </div>
                            </div>
                            {"In my third semester I started as an exercise group leader for the course Digital Systems I. \
                            Teaching first semester students the fundamtentals of computer architecture based on digital circuits, memory and finite state machines."}
                        </div>
                </div>
            </div>
            <div class="work-side col-12 col-lg-4 col-xl-3 px-lg-4 pb-lg-4">
                <h3 class="section-heading">{"Skills"}</h3>

                    <h4>{"Technical"}</h4>
                    <ul class="list-unstyled">
                        <li>{"Golang"}</li>
                        <li>{"Rust"}</li>
                        <li>{"Kubernetes"}</li>
                        <li>{"Cloud Infrastructure"}</li>
                        <li>{"Lead and deliver complex software systems"}</li>
                    </ul>

                    <h4>{"Professional"}</h4>
                    <ul class="list-unstyled">
                        <li>{"Effective communication"}</li>
                        <li>{"Team player"}</li>
                        <li>{"Strong problem solver"}</li>
                    </ul>

                <h3 class="section-heading">{"Education"}</h3>
                    <div>{"BSc in Computer Science"}</div>
                    <div>{"Christian-Albrechts-Universität Kiel"}</div>
                    <div>{"2008 - 2012"}</div>

                <h3 class="section-heading">{"Awards"}</h3>
                    <div>{"Koordinaten Festival für räumliche Medien - FH Kiel"}</div>
                    <div>{"3rd place and audience award"}<a href={"//vimeo.com/120791544"}>{"Video"} </a> </div>
                    <div>{"2014"}</div>
                <h3 class="section-heading">{"Languages"}</h3>
                    <ul class="list-unstyled">
                        <li>{"German"}<span class="text-muted">{"(Native)"}</span></li>
                        <li>{"English"}<span class="text-muted">{"(Professional)"}</span></li>
                    </ul>
                <h3 class="section-heading">{"Interests"}</h3>
                    <ul class="list-unstyled">
                        <li>{"Bouldering"}</li>
                        <li>{"Creative Coding"}</li>
                        <li>{"Travelling"}</li>
                        <li>{"Woodworking"}</li>
                    </ul>
            </div>
        </div>
    }
}