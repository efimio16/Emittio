import CTA from "@/components/sections/CTA";
import Comparison from "@/components/sections/Comparison";
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
                <Comparison/>
                <CTA/>
                <FAQ/>
            </main>
            <Footer/>
        </>
    );
}
