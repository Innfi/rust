#!/bin/bash
docker container stop tester
docker container rm tester
docker image rm tester:0.1.0
