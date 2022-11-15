# dataset came from https://data.opendatasoft.com/explore/dataset/geonames-postal-code@public/export/?flg=en
# and https://data.opendatasoft.com/explore/dataset/countries-codes%40public/table/
# we remove unused columns to reduce the size of the datasets

import csv
import sys

csv.field_size_limit(sys.maxsize)


# countries"
postal_code_country_code = []
filtered_country_code = ["PK", "IN", "MY", "TH", "SG", "JP",
                         "KR", "ZA", "PH", "PW", "RU", "BY", "RS", "UA", "MX", "BR"]

# geo postal code
cleaned = []
header = None

with open('./geonames-postal-code.csv') as f:
    reader = csv.reader(f, delimiter=';')
    for index, row in enumerate(reader):
        if index > 0:
            if row[0] not in filtered_country_code:
                if row[0] not in postal_code_country_code:
                    postal_code_country_code.append(
                        row[0])  # for filtering countries
                cleaned.append(";".join([row[0], row[1], row[2]]))
        else:
            header = ";".join([row[0], row[1], row[2]])

cleaned.sort()
if header:
    cleaned = [header] + cleaned
with open("./geonames-postal-code-filtered.csv", "w") as f:
    f.write("\n".join(cleaned))


# country code
# OFFICIAL LANG CODE;ISO2 CODE;ISO3 CODE;ONU CODE;IS ILOMEMBER;IS RECEIVING QUEST;LABEL EN;LABEL FR;LABEL SP;Geo Shape;geo_point_2d

cleaned = []
header = None
with open('./countries-codes.csv') as f:
    reader = csv.reader(f, delimiter=';')
    for index, row in enumerate(reader):
        if index > 0:
            if row[1] in postal_code_country_code:
                cleaned.append(";".join([row[1], row[6]]))
            else:
                print(f"filter country {row[1]}")
        else:
            header = ";".join([row[1], row[6]])
cleaned.sort()
if header:
    cleaned = [header] + cleaned
with open("./countries-codes-filtered.csv", "w") as f:
    f.write("\n".join(cleaned))
