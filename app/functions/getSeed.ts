function getSeed() {
    const  min = Math.ceil(0);
    const max = Math.floor(Math.pow(2, 64));
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

export default getSeed;