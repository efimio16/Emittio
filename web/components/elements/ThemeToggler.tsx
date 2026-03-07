'use client';

import { Moon, Sun } from "lucide-react";
import { useTheme } from "next-themes";
import { useEffect, useState } from "react";

export default function ThemeToggler() {
    const [mounted, setMounted] = useState(false);
    const { theme, setTheme } = useTheme();
    const [overlay, setOverlay] = useState<null | {
        x: number;
        y: number;
        color: string;
    }>(null);

    useEffect(() => {
        setMounted(true)
    }, []);

    useEffect(() => {
        if (!overlay) return;

        const timeout = setTimeout(() => setOverlay(null), 400);
        return () => clearTimeout(timeout);
    }, [overlay]);

    const toggle = (e: React.MouseEvent) => {
        const rect = (e.target as HTMLElement).getBoundingClientRect();

        setOverlay({
            x: rect.left + rect.width / 2,
            y: rect.top + rect.height / 2,
            color: theme === 'dark' ? 'white' : 'black',
        });

        setTimeout(() => setTheme(theme === 'dark' ? 'light' : 'dark'), 200);
    }

    if (!mounted) return null;

    return (
        <>
            <button
                onClick={toggle}
                className="p-3 cursor-pointer
                text-gray-800 dark:text-gray-200"
            >
                {theme === 'dark' ? <Moon/> : <Sun/>}
            </button>
            {overlay && (
                <div
                    className="fixed inset-0 z-9999 pointer-events-none"
                    style={{
                        background: overlay.color,
                        maskImage: `radial-gradient(circle at ${overlay.x}px ${overlay.y}px, white var(--r1), transparent var(--r2))`,
                        WebkitMaskImage: `radial-gradient(circle at ${overlay.x}px ${overlay.y}px, white var(--r1), transparent var(--r2))`,
                        animation: 'maskReveal 0.4s ease-out forwards',
                        maskRepeat: 'no-repeat',
                        WebkitMaskRepeat: 'no-repeat',
                    }}
                />
            )}
        </>
    )
}