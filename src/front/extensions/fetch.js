export default {
    methods: {
        $fetch(url, params, stringify = true) {
            let request = {
                method: 'get',
                credentials: 'include',
                mode: 'cors',
            };

            if (params !== undefined) {
                const headers = new Headers();
                if (stringify) {
                    headers.append('Content-Type', 'application/json');
                }

                Object.assign(request, {
                    headers: headers,
                    method: 'post',
                    body: stringify ? JSON.stringify(params) : params,
                });
            }

            return fetch('/api/' + url, request).then(resp => {
                const contentType = resp.headers.get('Content-Type');

                if (/(application|text)\/json/.test(contentType)) {
                    return resp.json().then(data => {
                        if (typeof data === 'object' && 'message' in data) {
                            if (Array.isArray(data.message)) {
                                data.message.forEach(message => this.$notify(message));
                            } else {
                                switch (resp.status) {
                                    case 403:
                                        this.$notify({
                                            type: 'error',
                                            text: data.message || "You have no access!",
                                        });
                                        break;
                                    case 404:
                                        this.$notify({
                                            type: 'error',
                                            text: data.message || 'Not found!',
                                        });
                                        break;
                                    default:
                                        if (data.message) {
                                            this.$notify({
                                                type: resp.ok ? 'success' : 'error',
                                                text: data.message,
                                            });
                                        }
                                }
                            }
                        }

                        if (resp.ok) {
                            return data;
                        } else {
                            if (resp.status === 401) {
                                this.$root.needLogin(); // Backend session doesn't exist. Login required.
                            }

                            // Hack to allow to get JSON another time;
                            const altResponse = {
                                ok: resp.ok,
                                header: resp.headers,
                                status: resp.status,
                                json: () => new Promise((resolve) => resolve(data)),
                            };

                            return Promise.reject(altResponse);
                        }
                    });
                } else {
                    return resp.text();
                }
            });
        },
    }
}
