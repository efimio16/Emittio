'use client';

import clsx from "clsx";
import { ReactElement, useEffect, useState } from "react";

interface MorphingToggleProps {
    active: number;
    children: ReactElement[];
    blur: number;
    className?: string;
}

export default function MorphingToggle(props: MorphingToggleProps) {
    const [blur, setBlur] = useState(false);
    const [morphing, setMorphing] = useState(false);

    useEffect(() => {
        setMorphing(true);
        setBlur(true);
        const timouts = [
            setTimeout(() => setBlur(false), 250),
            setTimeout(() => setMorphing(false), 500),
        ];
        return () => {
            for (const id of timouts) clearTimeout(id);
            setBlur(false);
        }
    }, [props.active]);

    return (
        <div
            className={clsx("relative grid", morphing && "filter-[url(#morphing)]", props.className)}
        >
            {props.children.map((children, i) =>
                <div
                    key={i}
                    className="col-start-1 row-start-1 flex items-center justify-center blur-none duration-500 transition-[opacity,filter,visibility] ease-out"
                    style={{ opacity: props.active === i ? '1' : '0', visibility: props.active === i ? 'visible' : 'hidden', filter: blur ? `blur(${props.blur}px)` : '' }}
                    data-blurred={blur}
                >
                    {children}
                </div>
            )}
        </div>
    )
}