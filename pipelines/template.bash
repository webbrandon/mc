#!/bin/bash 

rm install.params || :
echo "{" > install.params
echo "\"VERSION\": \""${VERSION}"\"" >> install.params
echo "}" >> install.params