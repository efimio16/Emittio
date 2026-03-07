"use client";

import { useEffect, useRef, useState } from "react";

type TypewriterTextProps = {
    children: string;
    speed?: number;
    delay?: number;
    cursor?: boolean;
    className?: string;
}

export function TypingAnimation({
    children,
    speed = 100,
    delay = 0,
    cursor = true,
    className = "",
}: TypewriterTextProps) {
    const [visibleText, setVisibleText] = useState("");
    const ref = useRef<HTMLSpanElement | null>(null);
    const [started, setStarted] = useState(false);

    useEffect(() => {
        const el = ref.current;
        if (!el || started) return;

        const observer = new IntersectionObserver(
            ([entry]) => {
                if (entry.isIntersecting) {
                setStarted(true);
                observer.disconnect();
                }
            },
            {
                threshold: 0.3,
            }
        );

        observer.observe(el);
        return () => observer.disconnect();
    }, [started]);

    useEffect(() => {
        if (!started) return;
        if (visibleText.length >= children.length) return;

        const timeout = setTimeout(() => {
            setVisibleText(children.slice(0, visibleText.length + 1));
        }, speed);

        return () => clearTimeout(timeout);
    }, [visibleText, children, speed, started]);

    return (
        <span ref={ref} className={`inline-flex items-center ${className}`}>
            <span>{visibleText}</span>

            {cursor && (
                <span
                    className="
                        ml-0.5 inline-block h-[1em] w-px
                        animate-pulse bg-current
                    "
                />
            )}
        </span>
    );
}