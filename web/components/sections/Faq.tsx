import { ChevronDownIcon } from "lucide-react";
import { useState } from "react";
import Description from "../elements/Description";
import { TypingAnimation } from "../elements/TypingAnimation";
import Heading from "../elements/Heading";

const faq = [
    {
        question: "Is this really anonymous?",
        answer: [
            "Yes. The system is designed so your identity is never collected or stored.",
            "There are no accounts, no personal data, and no way to link your aliases together.",
        ]
    },	
    {
        question: "Do I need to create an account?",
        answer: [
            "No.",
            "There is no signup, no username, and no password.",
            "Everything works without creating an account.",
        ]
    },	
    {
        question: "Can you see my emails?",
        answer: [
            "No.",
            "Messages are encrypted on your device before they are sent.",
            "Only the recipient can read them.",
        ]
    },	
    {
        question: "Is it free?",
        answer: [
            "Yes.",
            "Core features will always be free.",
            "We may introduce optional paid features later to support development.",
        ]
    },	
    {
        question: "How long are my emails stored?",
        answer: [
            "Messages are stored temporarily in the network for up to one week.",
            "This is not for privacy reasons — it helps prevent the network from being overloaded by millions of temporary emails, newsletters, and verification codes.",
            "You can still optionally save emails locally on your device for as long as you like.",
            "This way, you control your archive while the network stays fast and efficient.",
        ]
    },	
    {
        question: "Is it legal?",
        answer:[
            "Yes.",
            "This is a privacy-focused email system designed to protect users from ",
            "tracking and data collection.",
            "It does not enable or encourage illegal activity.",
        ]
    },	
    {
        question: "Is this related to cryptocurrencies?",
        answer: [
            "No.",
            "While the network is decentralized — similar to many cryptocurrencies — it has completely different goals and infrastructure.",
            "There are no tokens, no wallets, and no financial transactions.",
        ]
    },	
    {
        question: "Is it as simple as regular email?",
        answer: [
            "Yes.",
            "It's even simpler — there are no accounts, usernames, or passwords to remember.",
        ]
    },
    {
        question: "Can I email Gmail / Outlook?",
        answer: [
            "Yes.",
            "You can send and receive emails from traditional email services.",
        ]
    },	
    {
        question: "Can I switch from Gmail to your network?",
        answer: [
            "Yes.",
            "Email import and migration are planned, but may not be available in the first release.",
        ]
    },	
    {
        question: "Can I use multiple devices?",
        answer: [
            "Yes.",
            "You can access your aliases across devices and stay in sync with created or deleted aliases — without creating an account.",
        ]
    },
    {
        question: "What if I lose all my devices?",
        answer: [
            "You can recover your aliases using a recovery file that contains your ",
            "private seed.",
            "",
            "Please note:",
            "- This is the only way to recover access.",
            "- Store it securely and offline.",
            "- Anyone with this file has full control over your aliases.",
            "",
            "You are the only one responsible for your inbox.",
        ]
    },	
    {
        question: "Does it work with Tor?",
        answer: [
            "Yes.",
            "The system works over Tor.",
        ]
    },	
    {
        question: "Is it open source?",
        answer: [
            "Yes.",
            "The protocol and code are public, so anyone can verify how the system works.",
        ]
    },
    {
        question: "When will it be available?",
        answer: [
            "The project is under active development.",
            "You can join the waitlist to get early access.",
        ]
    },
];

export default function FaqSection() {
    const [open, setOpen] = useState<number>();
    
    return (
        <section className="mx-auto py-24 px-6 max-w-7xl w-full flex flex-col items-center">
            <Heading>
                <TypingAnimation cursor={false}>Questions? Answers.</TypingAnimation>
            </Heading>
            <div className="w-full divide-y divide-gray-200 dark:divide-gray-800">
                {faq.map(({ question, answer }, i) => 
                    <div
                        data-open={open === i || null}
                        key={i}
                        className="group w-full flex flex-col gap-3 mb-3 transition-color p-5 duration-150"
                        onClick={() => setOpen(open => open === i ? undefined : i)}
                    >
                        <p className="flex items-center justify-between">
                            <span className="text-lg">{question}</span>
                            <ChevronDownIcon className="size-5 flex-none group-data-open:rotate-180 transition-transform duration-300" />
                        </p>
                        <Description className="group-[&:not([data-open])]:hidden">
                            {answer.map((ln, i) => <span key={i}>{ln}<br/></span>)}
                        </Description>
                    </div>
                )}
            </div>
        </section>
    );
}