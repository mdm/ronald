export const hex = (value: number, width?: number) => {
    const str = value.toString(16);

    return width ? str.padStart(width, '0') : str;
};
