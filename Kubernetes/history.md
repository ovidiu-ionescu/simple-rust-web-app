# Install the app in Kubernetes
```bash
$ alias k=kubectl
```
Create the namespace.
```bash
k get namespaces
k create -f ./patelimon-namespace.yml
k describe namespaces pantelimon
```

Create the configmap
```bash
k create -f configmap.yml
k replace -f configmap.yml
k get configmaps -n pantelimon
k get cm web-config -n pantelimon -o yaml
```

Create the docker image inside minikube so we do not need a repository.
Might want to invoke bash to get rid of the env at the end.
```bash
eval $(minikube docker-env)
cd Docker
docker build --tag web-app:1 .
```

Create the deployment
```bash
k create -f deployment.yml
k replace -f deployment.yml
k get deployments -n pantelimon
k get pods -n pantelimon
k describe pod -n pantelimon web-app-deployment-595fc746b9-wfdjw
k logs -n pantelimon web-app-deployment-595fc746b9-wfdjw
```

Create the service.
```bash
k create -f service.yml
```
Service can not be replaced. If you want to alter it, you need to delete it.
```bash
k delete service web-service -n pantelimon
```

Get the internal IP address
```bash
k get pods -n pantelimon -o wide
```
Run inside the docker of minikube
```bash
docker exec -it 3a /bin/bash
curl http://172.17.0.8:8080/index
```

Access it from outside the cluster. First find the ip address of the cluster
```bash
k config view
curl http://192.168.49.2:30007/index
```

