#!/bin/bash
#set -x

# get all names and concatenate them with comma
NAMES="$(kubectl api-resources \
                   --namespaced \
                                    --verbs list \
                                                     -o name \
                                                                | tr '\n' ,)"

# ${NAMES:0:-1} -- because of `tr` command added trailing comma
# --show-kind is optional
kubectl get "${NAMES:0:-1}" --show-kind -n $1
