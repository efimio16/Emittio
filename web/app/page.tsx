import CTA from "@/components/sections/CTA";
import Difference from "@/components/sections/Difference";
import FAQ from "@/components/sections/FAQ";
import Footer from "@/components/sections/Footer";
import Header from "@/components/sections/Header";
import How from "@/components/sections/How";
import Why from "@/components/sections/Why";

export default function Main() {
    return (
        <>
            <main className="px-4">
                <Header/>
                <Why/>
                <How/>
                <Difference/>
                <CTA/>
                <FAQ/>
            </main>
            <Footer/>
        </>
    );
}
