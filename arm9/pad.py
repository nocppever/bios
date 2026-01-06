import os

filename = "bios9.bin"
target_size = 4096  # 4 Ko exacts

if not os.path.exists(filename):
    print(f"Erreur : {filename} introuvable.")
    exit(1)

with open(filename, "rb") as f:
    data = f.read()

current_size = len(data)
print(f"Taille actuelle : {current_size} octets")

if current_size < target_size:
    print(f"Ajout de padding pour atteindre {target_size} octets...")
    # On ajoute des zéros (0x00) jusqu'à la fin
    padding = b'\x00' * (target_size - current_size)
    with open(filename, "wb") as f:
        f.write(data + padding)
    print("Succès ! Fichier prêt pour MelonDS.")
    
elif current_size > target_size:
    print(f"ERREUR CRITIQUE : Le fichier dépasse 4 Ko ({current_size}) !")
    print("Il ne rentrera pas dans la mémoire.")
    
else:
    print("La taille est déjà parfaite (4096 octets).")