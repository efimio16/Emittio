'use client';

import Footer from "@/components/elements/Footer";
import Header from "@/components/elements/Header";
import FaqSection from "@/components/sections/Faq";
import FeaturesSection from "@/components/sections/Features";
import FirstCtaSection from "@/components/sections/FirstCta";
import HeroSection from "@/components/sections/Hero";
import HowItWorksSection from "@/components/sections/HowItWorks";
import PrinciplesSection from "@/components/sections/Principles";
import SecondCtaSection from "@/components/sections/SecondCta";
import { useEffect, useRef, useState } from "react";

export default function Main() {
    const sentinelRef = useRef<HTMLHRElement>(null);
    const [inHeaderZone, setInHeaderZone] = useState(false);

    useEffect(() => {
        if (!sentinelRef.current) return;

        const observer = new IntersectionObserver(([entry]) => {
            if (!entry.isIntersecting && entry.boundingClientRect.top < 0) setInHeaderZone(true);

            if (entry.isIntersecting) setInHeaderZone(false);
        });

        observer.observe(sentinelRef.current);
        return () => observer.disconnect();
    }, []);

    return (
        <>
            <Header showActions={inHeaderZone}/>
            <main className="flex-1 z-0">
                <HeroSection ref={sentinelRef} hideButtons={inHeaderZone}/>
                <PrinciplesSection/>
                <HowItWorksSection/>
                <FeaturesSection/>
                <FirstCtaSection/>
                <FaqSection/>
                <SecondCtaSection/>
            </main>
            <Footer/>
        </>
    )
}
