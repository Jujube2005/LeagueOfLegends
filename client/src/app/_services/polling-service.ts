class GlobalPolling {
    private interval: any = null
    private listeners: Function[] = []

    start() {
        if (this.interval) return

        this.interval = setInterval(async () => {
            try {
                const res = await fetch("/api/invite-count")
                if (res.ok) {
                    const data = await res.json()
                    this.listeners.forEach(fn => fn(data))
                }
            } catch (error) {
                // Silent fail for polling
            }
        }, 10000)
    }

    stop() {
        if (this.interval) {
            clearInterval(this.interval)
            this.interval = null
        }
    }

    subscribe(fn: Function) {
        this.listeners.push(fn)
        return () => {
            this.listeners = this.listeners.filter(l => l !== fn)
        }
    }
}

export const globalPolling = new GlobalPolling()

