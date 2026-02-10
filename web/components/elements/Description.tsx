import clsx from "clsx";

type DescriptionProps = {
    children: React.ReactNode;
    className?: string;
}

export default function Description({ children, className }: DescriptionProps) {
    return (
        <p
            className={clsx(
                "text-gray-600 dark:text-gray-500 text-lg font-light leading-relaxed max-w-prose mb-5",
                className
            )}
        >
            {children}
        </p>
    );
}