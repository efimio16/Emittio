import Description from "../elements/Description";
import Heading from "../elements/Heading";
import ViewOnGitHub from "../elements/ViewOnGitHub";

export default function SecondCtaSection() {
    return (
        <section className="flex flex-col justify-center items-center max-w-7xl mx-auto py-24 px-6">
            <Heading>View on GitHub</Heading>
            <Description>Read the protocol and source code.</Description>
            <div className="w-1/3 max-w-lg min-w-xs">
                <ViewOnGitHub/>
            </div>
        </section>
    )
}