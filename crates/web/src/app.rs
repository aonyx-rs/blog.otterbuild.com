use leptos::prelude::*;
use leptos_meta::*;

use crate::components::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // Set the document title
        <Title text="Welcome to Otter 🦦" />

        // Inject metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <div class="container mx-auto max-w-4xl py-8">

            <H1>"Introducing Otter 🦦"</H1>

            <P>
                "Otter is an opinionated framework for creating modern, cross-platform applications with Rust."
            </P>
            <P>
                "At its core, Otter is guided by a set of convictions about how modern software should be built:"
            </P>

            <List list_type=ListType::Ordered>
                <ListItem>
                    <P>
                        <strong>"We Believe That There Are Better Ways to Write Code"</strong>
                    </P>
                    <P>
                        "Over decades, the tech industry has developed best practices for creating maintainable, performant, and secure applications. These principles have proven their worth at scale, but they’re often inaccessible to smaller teams. Otter embeds these lessons directly into the framework, empowering developers of all sizes to work smarter and faster."
                    </P>
                </ListItem>
                <ListItem>
                    <P>
                        <strong>"We Believe Rust Is a Better Programming Language"</strong>
                    </P>
                    <P>
                        "Rust’s powerful type system provides strong guarantees about code correctness, safety, and reliability, allowing developers to catch most potential errors during development rather than in production."
                    </P>

                </ListItem>
                <ListItem>
                    <P>
                        <strong>"We Believe Opinionated Frameworks Are Better"</strong>
                    </P>
                    <P>
                        "Opinionated frameworks solve foundational problems once, so developers can focus on what matters most—creating value. By offering clear conventions and a comprehensive toolset, Otter eliminates decision fatigue and accelerates the path from idea to execution."
                    </P>
                </ListItem>
            </List>

            <P>
                "At the end of the day, building apps shouldn’t require large teams of specialized developers. "
                <strong>
                    "With Otter, we’re combining the prototyping speed of Ruby on Rails with the cross-platform reach of Expo."
                </strong>
                " Our mission is to empower small teams to build profitable businesses and deliver meaningful applications to their users—wherever they are, on any platform."
            </P>

            <H2>"Roadmap"</H2>
            <P>
                "Building Otter is an ambitious undertaking, and we’re embracing the journey step by step. While the vision is clear, there are still many foundations to lay and challenges to navigate. With that in mind, here’s a rough outline of the roadmap we’ll follow to bring Otter to life."
            </P>

            <H3>"Web Applications"</H3>
            <P>
                "We’ll begin by prototyping Otter’s command-line interface (CLI), focusing initially on client-side rendered web applications. The goal is to provide developers with the tools they need to get started quickly and efficiently. This phase will include:"
            </P>
            <List list_type=ListType::Unordered>
                <ListItem>"Generating a new static website to serve as a starting point."</ListItem>
                <ListItem>
                    "Adding generators for a pre-made component library to accelerate UI development."
                </ListItem>
                <ListItem>
                    "Introducing routing and support for building multi-page web applications."
                </ListItem>
            </List>

            <H3>"Backend & API"</H3>
            <P>
                "Once the web application prototype is stable, the next step is to introduce backend functionality. This includes:"
            </P>
            <List list_type=ListType::Unordered>
                <ListItem>
                    "Seamless integration with a Postgres database for managing data."
                </ListItem>
                <ListItem>
                    "Support for RESTful APIs to enable communication between the frontend and backend."
                </ListItem>
            </List>

            <H2>"Authentication & Authorization"</H2>
            <P>
                "Security is a cornerstone of modern applications. During this phase, we’ll add:"
            </P>
            <List list_type=ListType::Unordered>
                <ListItem>"Authentication systems to manage user access."</ListItem>
                <ListItem>
                    "Authorization capabilities to ensure users only access the resources they’re permitted to."
                </ListItem>
            </List>

            <H2>"Developer Experience and DevOps"</H2>
            <P>
                "A great framework is only as good as the developer experience it offers. This phase will focus on:"
            </P>
            <List list_type=ListType::Unordered>
                <ListItem>
                    "Streamlining the development workflow to make Otter intuitive and enjoyable to use."
                </ListItem>
                <ListItem>
                    "Providing tools and integrations for deployment, testing, and CI/CD pipelines to ensure seamless operations."
                </ListItem>
            </List>

            <H2>"Mobile Apps"</H2>
            <P>
                "Finally, we’ll enable cross-platform mobile application development with Otter. This phase will unlock the ability to write mobile apps alongside web and backend code, ensuring a unified development experience across all platforms."
            </P>
        </div>
    }
}
