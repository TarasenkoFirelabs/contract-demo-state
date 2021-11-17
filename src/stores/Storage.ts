interface IStorage {
    getItem(key: string): string | null;
    setItem(key: string, value: string): void;
    removeItem(key: string): void;
}

abstract class Storage<T extends string> {
    private readonly storage: null | IStorage;

    public constructor(getStorage = (): null | IStorage => (typeof window !== 'undefined') ? window.localStorage : null) {
        this.storage = getStorage()
    }

    protected get(key: T, data = {}): string | null {
        if (this.storage) {
            const v = this.storage.getItem(key)
            try {
                return JSON.parse(v || JSON.stringify(data))
            } catch (e) {
                return v
            }
        }
        return null;
    }

    protected set(key: T, value: string): void {
        if (this.storage) {
            this.storage.setItem(key, value);
        }
    }

    protected clearItem(key: T): void {
        if (this.storage) {
            this.storage.removeItem(key);
        }
    }

    protected clearItems(keys: T[]): void {
        keys.forEach((key) => this.clearItem(key));
    }
}

export default Storage;

// export const get = (k, d = {}) => {
//     let v = localStorage.getItem(k)
//     try {
//         return JSON.parse(v || JSON.stringify(d))
//     } catch (e) {
//         return v
//     }
// }
// export const set = (k, v) => localStorage.setItem(k, typeof v === 'string' ? v : JSON.stringify(v))
// export const del = (k) => localStorage.removeItem(k)