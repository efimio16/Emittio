import Description from "@/components/elements/Description";
import Footer from "@/components/elements/Footer";
import Header from "@/components/elements/Header";
import Heading from "@/components/elements/Heading";

export default function Privacy() {
    return (
        <>
            <Header showActions/>
            <main className="px-4 mt-28 max-w-7xl mx-auto min-h-svh">
                <Heading>Have questions or feedback?</Heading>
                <Description>
                    Reach out to us at: <a href="mailto:emittio@proton.me">emittio@proton.me</a><br/>
                    (Official branded email will be available soon.)<br/><br/>

                    No personal information required — you can remain anonymous.
                </Description>
            </main>
            <Footer/>
        </>
    )
}
