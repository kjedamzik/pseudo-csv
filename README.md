pseudo-csv
==========

![release](https://github.com/kjedamzik/pseudo-csv/actions/workflows/release.yml/badge.svg)

pseudo-csv is a command line tool for GDPR compliant data pseudonymization in csv files.

pseudo-csv encrypts and base64 encodes selected columns using symmetric AES-128-CBC encryption and a key, derived from a passphrase, provided via `PSEUDO_CSV_PASSPHRASE` environment variable.


```
cat data.csv |xsv table
 email                 caterogy   revenue  date
 ada.doe@example.com   furniture  111.11   2021-01-01
 john.doe@example.com  groceries  222.22   2021-02-01
 elvis@example.com     toys       555.55   2021-03-01


export PSEUDO_CSV_PASSPHRASE="very secret passphrase!"

cat data.csv  |pseudo-csv email,caterogy |xsv table
 email                                         caterogy                  revenue  date
 uho/96MqvhzNjn35cSYSbk9ZST64mQZbx0iZ1Fl/1g0=  RZzZYSvA0QJlKM+RK78Csg==  111.11   2021-01-01
 VHJQ1Vv3gMeGiygka+AfBHMtZnq46LB2LsOC46ZKfas=  AHzyvPCbSpchpiEodwBtpQ==  222.22   2021-02-01
 Es34Hzzmdetdu3MMdmnwhG+0AVhjZzbfscK8DpPtxaw=  thfm3SvnAdrsrfZKvgxNcw==  555.55   2021-03-01

```

